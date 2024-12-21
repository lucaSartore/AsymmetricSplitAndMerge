use std::{borrow::BorrowMut, iter::Enumerate, ptr::null};

use super::*;

impl<'a, S: SplitterTrait, M: MergerTrait, L: LoggerTrait> MainLogic<'a, S, M, L, MergeState> {

    pub fn execute_merge(mut self, num_of_workers: usize) -> MainLogic<'a, S, M, L, CompleateState>{
        
        self.initialize();

        let handlers = self.launch_threads(num_of_workers);

        loop{
            let num_requests = self.send_merge_request();

            if num_requests == 0{
                break;
            }

            self.receive_merge_result(num_requests);
        }

        return MainLogic{
            image: self.image,
            splitter: self.splitter,
            merger: self.merger,
            logger: self.logger,
            split_tree: self.split_tree,
            state: CompleateState{}
        }
    }

    pub fn send_merge_request(&mut self) -> usize{
        
        let mut num_requests = 0;

        for id_a in self.state.disjoint_sets.get_root_items(){
            for id_b in self.state.disjoint_sets.get_set(*id_a)
                .expect("error merge phase")
                .get_neighbors(){

                // only comare them based on a certain order to avoid duplication
                if *id_b > *id_a {
                    continue
                }

                // do not check for stuff that has already been check
                if self.state.already_checked_mgerges.contains(&[*id_a, *id_b]){
                    continue
                }

                self.state.already_checked_mgerges.insert([*id_a, *id_b]);

                let mat_a = self.state.areas
                    .get_mut(&id_a)
                    .expect("error in area creation of merge phase")
                    .get_mat_area(&self.image.image);
                let mat_a = unsafe{UnmanagedMat::from_mat(mat_a)};

                let mat_b = self.state.areas
                    .get_mut(&id_b)
                    .expect("error in area creation of merge phase")
                    .get_mat_area(&self.image.image);
                let mat_b = unsafe{UnmanagedMat::from_mat(mat_b)};

                self.state.images_to_merge_tx.send((mat_a, *id_a, mat_b, *id_b))
                    .expect("child threads has failed");
                num_requests += 1;
            }
        }
        return num_requests;
    }

    pub fn initialize(&mut self){
        let mut v = Vec::new();
        self.split_tree[0].collect_leafs(&self.split_tree, &mut v);

        let mut max_id = 0;

        for (id,area) in &v {
            max_id = usize::max(max_id, *id);

            self.state.disjoint_sets.add_item(*id)
                .expect("something is wrong with the initialization of the merege state");
            self.state.areas.insert(*id, Area::new_from_split(*id, area));
        }
        self.state.next_area_id = max_id + 1;

        for i in 0..v.len(){
            for j in (i+1)..v.len(){
                let (a_id, a_mat) = v[i];
                let (b_id, b_mat) = v[j];
                if ImageContainerSplit::are_neighbors(a_mat, b_mat){
                    self.state.disjoint_sets.set_as_neighbors(a_id, b_id)
                        .expect("something is wrong with the initialization of the merege state");
                }
            }
        }
    }

    fn launch_threads(&self, num_of_workers: usize) -> Vec<JoinHandle<Result<()>>>{
        todo!()
    }

    fn receive_merge_result(&mut self, to_receive: usize){
        for _ in 0..to_receive {
            let (to_merge, id_a, id_b) = self.state.merge_result_rx.recv()
                .expect("child thread has fail");

            if !to_merge {
                continue
            }

            let new_item_id = self.state.next_area_id;
            self.state.next_area_id += 1;

            let area_a = self.state.areas.get_mut(&id_a)
                .expect("error while reading mask")
                .get_mat_area(&self.image.image) as *const Mat;
            let area_b = self.state.areas.get_mut(&id_b)
                .expect("error while reading mask")
                .get_mat_area(&self.image.image) as *const Mat;

            // the reference are still valid since we haven't touch the hashmap
            // (there is a mutable borrow only for the call to get_mat_area)
            let area_a = unsafe{&*area_a};
            let area_b = unsafe{&*area_b};
            
            let marker = AreaMarker::merge(area_a, area_b)
                .expect("error in creation of merge marker");

            let area = Area::new_from_id_and_marker(new_item_id, marker);

            self.state.areas.insert(new_item_id, area);
            
            self.state.disjoint_sets.create_new(new_item_id, [id_a,id_b])
                .expect("error in recieve merge result")
        }
    }
}
