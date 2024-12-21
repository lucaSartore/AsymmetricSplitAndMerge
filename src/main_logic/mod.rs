#[cfg(test)]
mod test;
mod split_phase;
mod split_tree;
mod merge_phase;
mod disjoint_set;

use split_tree::*;

use crate::prelude::*;
use std::{ sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}};



pub struct SplitState {
    yet_to_split_images: Vec<usize>,
    images_to_split_tx: Sender<(UnmanagedMat, usize)>,
    images_to_split_rx: Arc<Mutex<Receiver<(UnmanagedMat, usize)>>>,
    split_result_tx: Arc<Mutex<Sender<Option<(CutDirection, i32, usize)>>>>,
    split_result_rx: Receiver<Option<(CutDirection, i32, usize)>>,
    items_in_queue: usize,
}
pub struct MergeState {}

pub trait SplitMergeState {}
impl SplitMergeState for SplitState {}
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



