use super::*;

impl<'a, S: SplitterTrait, M: MergerTrait, L: LoggerTrait> MainLogic<'a, S, M, L, MergeState> {
    pub fn execute_merge(
        mut self,
        num_of_workers: usize,
    ) -> MainLogic<'a, S, M, L, CompleateState> {
        self.initialize();

        let join_handlers = self.launch_threads(num_of_workers);

        loop {
            info!("start sending split requests");
            let num_requests = self.send_merge_request();
            info!("main thread: sent {num_requests} requests");

            if num_requests == 0 {
                break;
            }

            info!("start receiving split requests");
            self.receive_merge_result(num_requests);
        }


        info!("Start thread join");
        join_handlers.into_iter().for_each(|x| {
            let _ = x.join().expect("one of the threads has exited unsuccessfully");
        });
        info!("Done thread join");

        return MainLogic {
            image: self.image,
            splitter: self.splitter,
            merger: self.merger,
            logger: self.logger,
            split_tree: self.split_tree,
            state: CompleateState {},
        };
    }

    pub fn send_merge_request(&mut self) -> usize {
        let mut num_requests = 0;

        for id_a in self.state.disjoint_sets.get_root_items() {
            for id_b in self
                .state
                .disjoint_sets
                .get_set(*id_a)
                .expect("error merge phase")
                .get_neighbors()
            {
                // only comare them based on a certain order to avoid duplication
                if *id_b > *id_a {
                    continue;
                }

                // do not check for stuff that has already been check
                if self.state.already_checked_mgerges.contains(&[*id_a, *id_b]) {
                    continue;
                }

                if !(self.state.disjoint_sets.is_root_item(*id_a) &&
                   self.state.disjoint_sets.is_root_item(*id_b)) {
                    continue
                }

                self.state.already_checked_mgerges.insert([*id_a, *id_b]);

                let mat_a = self
                    .state
                    .areas
                    .get_mut(&id_a)
                    .expect("error in area creation of merge phase")
                    .get_mat_area(&self.image.image);
                let mat_a = unsafe { UnmanagedMat::from_mat(mat_a) };

                let mat_b = self
                    .state
                    .areas
                    .get_mut(&id_b)
                    .expect("error in area creation of merge phase")
                    .get_mat_area(&self.image.image);
                let mat_b = unsafe { UnmanagedMat::from_mat(mat_b) };

                self.state
                    .images_to_merge_tx
                    .send((mat_a, *id_a, mat_b, *id_b))
                    .expect("child threads has failed");
                num_requests += 1;
            }
        }
        return num_requests;
    }

    pub fn initialize(&mut self) {
        let mut v = Vec::new();
        self.split_tree[0].collect_leafs(&self.split_tree, &mut v);

        let mut max_id = 0;

        for (id, area) in &v {
            max_id = usize::max(max_id, *id);

            self.state
                .disjoint_sets
                .add_item(*id)
                .expect("something is wrong with the initialization of the merege state");
            self.state
                .areas
                .insert(*id, Area::new_from_split(*id, area));
        }
        self.state.next_area_id = max_id + 1;

        for i in 0..v.len() {
            for j in (i + 1)..v.len() {
                let (a_id, a_mat) = v[i];
                let (b_id, b_mat) = v[j];
                if ImageContainerSplit::are_neighbors(a_mat, b_mat) {
                    info!("set as neighbors {a_id} {b_id}");
                    self.state
                        .disjoint_sets
                        .set_as_neighbors(a_id, b_id)
                        .expect("something is wrong with the initialization of the merege state");
                }
            }
        }
    }

    fn launch_threads(&self, num_of_workers: usize) -> Vec<JoinHandle<Result<()>>> {
        let mut join_handlers = Vec::new();
        for i in 0..num_of_workers {
            let rx = self.state.images_to_merge_rx.clone();
            let tx = self.state.merge_result_tx.clone();
            // I am sure that this reference will stay valid for as long as the thread below exist
            let merger: &'static M = unsafe { std::mem::transmute(&self.merger) };
            // let image: &'static Mat = unsafe { std::mem::transmute(&self.image) };
            let image = self.image.image.clone();
            join_handlers.push(thread::spawn(move || -> Result<()> {
                info!("thread {i} started");
                loop {
                    info!("thread {i} rx lock");
                    let rx_locked = rx.lock().map_err(|_| anyhow!("main tread has fail"))?;
                    info!("thread {i} rx locked");

                    let (img_a, id_a, img_b, id_b) = rx_locked.recv()?;
                    info!("thread {i} receive id=[{id_a},{id_b}]");

                    // let merge_result = true;
                    let merge_result = merger.merge(&img_a.image, &img_b.image, &image);
                    info!("thread {i} merge result = {:?}", merge_result);

                    info!("thread {i} tx lock");
                    let tx_lock = tx.lock().map_err(|_| anyhow!("main tread has fail"))?;
                    info!("thread {i} tx locked");

                    tx_lock.send((merge_result, id_a, id_b))
                        .expect("main thread has reash");

                    img_a.destroy();
                    img_b.destroy();
                    info!("thread {i} successfly processed id=[{id_a},{id_b}]");
                }
            }));
        }
        return join_handlers;
    }

    fn receive_merge_result(&mut self, to_receive: usize) {
        for _ in 0..to_receive {

            let (to_merge, id_a, id_b) = self
                .state
                .merge_result_rx
                .recv()
                .expect("child thread has fail");

            if !to_merge {
                continue;
            }
            
            let new_item_id = self.state.next_area_id;
            self.state.next_area_id += 1;

            let id_a = self.state.disjoint_sets.get_father_of(id_a)
                .expect("error in receive merge result");
            let id_b = self.state.disjoint_sets.get_father_of(id_b)
                .expect("error in receive merge result");

            let area_a = self
                .state
                .areas
                .get_mut(&id_a)
                .expect("error while reading mask")
                .get_mat_area(&self.image.image) as *const Mat;
            let area_b = self
                .state
                .areas
                .get_mut(&id_b)
                .expect("error while reading mask")
                .get_mat_area(&self.image.image) as *const Mat;

            // the reference are still valid since we haven't touch the hashmap
            // (there is a mutable borrow only for the call to get_mat_area)
            let area_a = unsafe { &*area_a };
            let area_b = unsafe { &*area_b };


            let marker =
                AreaMarker::merge(area_a, area_b).expect("error in creation of merge marker");

            let area = Area::new_from_id_and_marker(new_item_id, marker);

            self.logger.log_merge(new_item_id, [id_a,id_b])
                .expect("logger has failed");

            self.state.areas.insert(new_item_id, area);

            self.state
                .disjoint_sets
                .create_new(new_item_id, [id_a, id_b])
                .expect("error in recieve merge result");
        }
    }
}
