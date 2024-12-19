use std::marker::PhantomData;

use crate::logger_trait::LoggerTrait;
use crate::merger_trait::MergerTrait;
use crate::splitter_trait::SplitterTrait;

/// the main logic that contains all the code that is general for all variant of the split and
/// merge algorithm.
/// All the code that can vary from an implementation to another is encapsulated in the 3 generic
/// types `S`, `M` and `L`
pub struct MainLogic<S: SplitterTrait, M: MergerTrait, L: LoggerTrait>{
    x: PhantomData<(S,M,L)>
}
