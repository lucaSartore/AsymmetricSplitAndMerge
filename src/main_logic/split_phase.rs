use super::*;

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

        info!("Start thread spawning");
        let join_handlers = self.launch_threads(num_of_workers);
        info!("Done with thread spawning");

        info!("Starting execute split loop");
        loop {
            // send all necessary split requests
            info!("main thread: start sending");
            while self.send_split_request() {
                info!{"main thread send one piece"}
            };
            info!("main thread: end sending");
            // if we no longer have anything to 
            if self.state.items_in_queue == 0 {
                break;
            }
            // elaborate eventual received results
            info!("main thread: start receive");
            self.receive_split_result();
            info!("main thread: end receive");
        }

        info!("Exited execute split loop");

        drop(self.state.images_to_split_tx);
        drop(self.state.split_result_rx);

        info!("Start thread join");
        join_handlers.into_iter().for_each(|x| {
            let _ = x.join().expect("one of the threads has exited unsuccessfully");
        });
        info!("Done thread join");
    
        return MainLogic{
            image: self.image,
            splitter: self.splitter,
            merger: self.merger,
            logger: self.logger,
            state: MergeState::default(),
            split_tree: self.split_tree
        }

    }

    fn launch_threads(&self, num_of_workers: usize) -> Vec<JoinHandle<Result<()>>>{
        let mut join_handlers = Vec::new();
        for i in 0..num_of_workers {
            let rx = self.state.images_to_split_rx.clone();
            let tx = self.state.split_result_tx.clone();
            // I am sure that this reference will stay valid for as long as the thread below exist
            let splitter: &'static S = unsafe {std::mem::transmute(&self.splitter)};
            join_handlers.push(thread::spawn(move || -> Result<()>{
                info!("thread {i} started");
                loop {
                    info!("thread {i} rx lock");
                    let rx_locked = rx.lock().map_err(|_|  anyhow!("main tread has fail"))?;
                    info!("thread {i} rx locked");

                    let (img, id) = rx_locked.recv()?;
                    info!("thread {i} receive id={id}");

                    let split_result = splitter.split(&img.image);
                    info!("thread {i} split result = {:?}", split_result);

                    info!("thread {i} tx lock");
                    let tx_lock = tx.lock().map_err(|_|  anyhow!("main tread has fail"))?;
                    info!("thread {i} tx locked");
                    
                    if let Some((direction, split_at)) = split_result{
                        tx_lock.send(Some((direction, split_at, id))).expect("send messages should never fail");
                    }else{
                        tx_lock.send(None).expect("send messages should never fail");
                    }
                    img.destroy();
                    info!("thread {i} successfly processed id={id}");
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

        info!("main thread send request for id={to_split_id}");

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

        info!{"start listening with {} elements",self.state.items_in_queue};
        let result = self.state.split_result_rx.recv()
            .expect("there should always be a thread listening");
        info!{"got result: {:?}",result};

        self.state.items_in_queue -= 1;

        let (direction, split_at, id_splitted) = match result {
            Some(e) => e,
            _ => return
        };

        let id_1 = self.split_tree.len();
        let id_2 = id_1 + 1;
        
        let [img_1, img_2] = self.split_tree[id_splitted].image.split(direction, split_at)
            .expect( &format!(
                "the splitter has givven returned an invalid split configuraton: direction={:?} split_at={} ",
                direction,
                split_at
            ));

        self.logger.log_split(
            id_splitted,
            [
                Area::new_from_split(id_1, &img_1),
                Area::new_from_split(id_2, &img_2)
            ]
        ).expect("logger trait has failed");

        unsafe{
            // the value is already borrowed immutably  by the split, therefore pushing should not
            // be safe (as we might move some elements around) however in this case the lifetime of
            // the  split tree is only referencing the c++ heap allocated object, and not any data
            // inside the vector, therefore we can do this
            let split_tree_ptr = &self.split_tree as *const Vec<_> as *mut Vec<SplitTree<'_>>;
            (*split_tree_ptr)[id_splitted].childs = Some([id_1, id_2]);
            (*split_tree_ptr).push(SplitTree::new(id_1, img_1));
            (*split_tree_ptr).push(SplitTree::new(id_2, img_2));
        }

        self.state.yet_to_split_images.push(id_1);
        self.state.yet_to_split_images.push(id_2);
    }

}
