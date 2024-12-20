#[cfg(test)]
mod test;


use crate::image_container::*;
use crate::logger_trait::LoggerTrait;
use crate::merger_trait::MergerTrait;
use crate::splitter_trait::SplitterTrait;
use std::{ sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}};
use anyhow::{anyhow,Result};


pub trait SplitMergeState {}
struct SplitState {
    yet_to_split_images: Vec<usize>,
    images_to_split_tx: Sender<(UnmanagedMat, usize)>,
    images_to_split_rx: Arc<Mutex<Receiver<(UnmanagedMat, usize)>>>,
    split_result_tx: Arc<Mutex<Sender<Option<(CutDirection, i32, usize)>>>>,
    split_result_rx: Receiver<Option<(CutDirection, i32, usize)>>,
    items_in_queue: usize,
}
impl SplitMergeState for SplitState {}

struct MergeState {}
impl SplitMergeState for MergeState {}

/// the main logic that contains all the code that is general for all variant of the split and
/// merge algorithm.
/// All the code that can vary from an implementation to another is encapsulated in the 3 generic
/// types `S`, `M` and `L`
pub struct MainLogic<'a, S: SplitterTrait, M: MergerTrait, L: LoggerTrait, ST: SplitMergeState> {
    splitter: S,
    merger: M,
    logger: L,
    state: ST,
    image: &'a ImageContainer,
    split_tree: Vec<SplitTree<'a>>,
}

impl<'a, S: SplitterTrait, M: MergerTrait, L: LoggerTrait> MainLogic<'a, S, M, L, SplitState> {
    pub fn new(splitter: S, merger: M, logger: L, image: &'a ImageContainer) -> Self {

        let image_split = image.to_image_container_split();
        let split_tree = vec![SplitTree {
            id: 0,
            image: image_split,
            childs: None,
        }];

        let (images_to_split_tx, images_to_split_rx) = channel();
        let (split_result_tx, split_result_rx) = channel();
        let state = SplitState {
            yet_to_split_images: vec![0],
            images_to_split_tx,
            images_to_split_rx: Arc::new(Mutex::new(images_to_split_rx)), 
            split_result_tx: Arc::new(Mutex::new(split_result_tx)), 
            split_result_rx,
            items_in_queue: 0
        };

        return MainLogic {
            splitter,
            merger,
            logger,
            image,
            split_tree,
            state,
        };
    }


    pub fn execute_split(mut self, num_of_workers: usize) -> MainLogic<'a, S, M, L, MergeState>{

        let join_handlers = self.launch_threads(num_of_workers);

        loop {
            // send all necessary split requests
            while !self.send_split_request() {};
            // if we no longer have anything to 
            if self.state.items_in_queue == 0 {
                break;
            }
            // elaborate eventual received results
            self.receive_split_result();
        }

        join_handlers.into_iter().for_each(|x| {
            let _ = x.join().expect("one of the threads has exited unsuccessfully");
        });
    
        return MainLogic{
            image: self.image,
            splitter: self.splitter,
            merger: self.merger,
            logger: self.logger,
            state: MergeState{},
            split_tree: self.split_tree
        }

    }

    fn launch_threads(&self, num_of_workers: usize) -> Vec<JoinHandle<Result<()>>>{
        let mut join_handlers = Vec::new();
        for _ in 0..num_of_workers {
            let rx = self.state.images_to_split_rx.clone();
            let tx = self.state.split_result_tx.clone();
            // I am sure that this reference will stay valid for as long as the thread below exist
            let splitter: &'static S = unsafe {std::mem::transmute(&self.splitter)};
            join_handlers.push(thread::spawn(move || -> Result<()>{
                loop {
                    let img = rx.lock().map_err(|_|  anyhow!("main tread has fail"))?;
                    let (img, id) = img.recv()?;
                    let split_result = splitter.split(&img.image);
                    let tx_lock = tx.lock().map_err(|_|  anyhow!("main tread has fail"))?;
                    if let Some((direction, split_at)) = split_result{
                        tx_lock.send(Some((direction, split_at, id)))?;
                    }else{
                        tx_lock.send(None)?;
                    }
                }
            }));
        }
        return join_handlers
    }

    fn send_split_request(&mut self) -> bool{
        // extract the image that need to be split
        let to_split_id = self.state.yet_to_split_images.pop();
        let to_split_id = match to_split_id {
            Some(e) => e,
            _ => return false
        };
        let to_split = &self.split_tree[to_split_id].image;
        let to_split = unsafe{ UnmanagedMat::from_image_container_split(to_split) };

        // send the image 
        self.state.images_to_split_tx.send((to_split, to_split_id))
            .expect("there should always be a thread listening");
        self.state.items_in_queue += 1;
        return true
    }

    fn receive_split_result(&mut self){
        
        assert_ne!(self.state.items_in_queue,0,"can't receive a message if there are no items in the queue, as doing so would deadlock the program");

        let result = self.state.split_result_rx.recv()
            .expect("there should always be a thread listening");

        self.state.items_in_queue -= 1;

        let (direction, split_at, id) = match result {
            Some(e) => e,
            _ => return
        };

        let id_1 = self.split_tree.len();
        let id_2 = id_1 + 1;
        
        let [img_1, img_2] = self.split_tree[id].image.split(direction, split_at)
            .expect( &format!(
                "the splitter has givven returned an invalid split configuraton: direction={:?} split_at={} ",
                direction,
                split_at
            ));

        unsafe{
            // the value is already borrowed immutably  by the split, therefore pushing should not
            // be safe (as we might move some elements around) however in this case the lifetime of
            // the  split tree is only referencing the c++ heap allocated object, and not any data
            // inside the vector, therefore we can do this
            let split_tree_ptr = &self.split_tree as *const Vec<_> as *mut Vec<SplitTree<'_>>;
            (*split_tree_ptr)[id].childs = Some([id_1, id_2]);
            (*split_tree_ptr).push(SplitTree::new(id_1, img_1));
            (*split_tree_ptr).push(SplitTree::new(id_2, img_2));
        }

    }

}

struct SplitTree<'a> {
    id: usize,
    pub image: ImageContainerSplit<'a>,
    childs: Option<[usize; 2]>,
}

impl<'a> SplitTree<'a> {
    fn new(id: usize, image: ImageContainerSplit<'a>) -> Self{
        return Self{
            id,
            image,
            childs: None
        };
    } 
}
