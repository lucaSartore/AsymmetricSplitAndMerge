#[cfg(test)]
mod test;
mod split_phase;
mod split_tree;
mod merge_phase;
mod disjoint_set;

use disjoint_set::*;
use split_tree::*;

use crate::prelude::*;
use std::{ collections::{HashMap, HashSet}, sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}};



pub struct SplitState {
    yet_to_split_images: Vec<usize>,
    images_to_split_tx: Sender<(UnmanagedMat, usize)>,
    images_to_split_rx: Arc<Mutex<Receiver<(UnmanagedMat, usize)>>>,
    split_result_tx: Arc<Mutex<Sender<Option<(CutDirection, i32, usize)>>>>,
    split_result_rx: Receiver<Option<(CutDirection, i32, usize)>>,
    items_in_queue: usize,
}

#[derive(Debug)]
pub struct MergeState {
    disjoint_sets: DisjointSets,
    images_to_merge_tx: Sender<(UnmanagedMat, usize, UnmanagedMat, usize)>,
    images_to_merge_rx: Arc<Mutex<Receiver<(UnmanagedMat, usize, UnmanagedMat, usize)>>>,
    merge_result_tx: Arc<Mutex<Sender<(bool, usize, usize)>>>,
    merge_result_rx: Receiver<(bool, usize, usize)>,
    already_checked_mgerges: HashSet<[usize;2]>,
    areas: HashMap<usize, Area>,
    next_area_id: usize,
}

impl Default for MergeState {
    
    fn default() -> Self {

        let (images_to_merge_tx, images_to_merge_rx) = channel();
        let (merge_result_tx, merge_result_rx) = channel();

        let images_to_merge_rx = Arc::new(Mutex::new(images_to_merge_rx));
        let merge_result_tx = Arc::new(Mutex::new(merge_result_tx));
       
        Self{
            images_to_merge_tx,
            images_to_merge_rx,
            merge_result_tx,
            merge_result_rx,
            next_area_id: 0,
            disjoint_sets: DisjointSets::default(),
            already_checked_mgerges: HashSet::default(),
            areas: HashMap::default()
        }
    }
}

pub struct CompleateState {}

pub trait SplitMergeState {}
impl SplitMergeState for SplitState {}
impl SplitMergeState for MergeState {}
impl SplitMergeState for CompleateState {}

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



