use super::*;

impl<'a, S: SplitterTrait, M: MergerTrait, L: LoggerTrait> MainLogic<'a, S, M, L, MergeState> {

    pub fn execute_split(mut self, num_of_workers: usize) -> MainLogic<'a, S, M, L, MergeState>{
        todo!()
    }

    fn launch_threads(&self, num_of_workers: usize) -> Vec<JoinHandle<Result<()>>>{
        todo!()
    }

    fn send_merge_request(&mut self) -> bool{
       todo!() 
    }

    fn receive_merge_result(&mut self){
    }
}
