use std::{borrow::BorrowMut, cell::RefCell, collections::{HashMap, HashSet}};
use crate::prelude::*;


pub struct DisjointSets{
    items: HashMap<usize, DisjointSet>,
    root_items: HashSet<usize>
}

impl DisjointSets{

    pub fn get_set(&self, id: usize) -> Option<&DisjointSet>{
        return self.items.get(&id);
    }

    pub fn create_new(&mut self, new_item_id: usize, childrends: [usize;2]) -> Result<()> {

        let [c1,c2] = childrends; 

        if self.root_items.get(&c1).is_none() || self.root_items.get(&c2).is_none(){
            return Err(anyhow!("impossible to merge tow non root IDs"));
        }
        
        self.items[&c1].set_father(new_item_id);
        self.items[&c2].set_father(new_item_id);
        
        self.items.insert(new_item_id, DisjointSet{
            id: new_item_id,
            set_father: RefCell::new(new_item_id),
            neighbors: Vec::new()
        });

        let neighbors = self.items[&c1].neighbors.iter()
            .chain(self.items[&c2].neighbors.iter())
            .filter(|x| {
                self.items[&x].get_father(self) != new_item_id
            })
            .map(|x| *x)
            .collect();

        self.items.get_mut(&new_item_id)
            .expect("error in code of disjoint set building")
            .neighbors = neighbors;

        self.root_items.remove(&c1);
        self.root_items.remove(&c2);
        self.root_items.insert(new_item_id);

        Ok(())
    }

    pub fn add_item(&mut self, id: usize) -> Result<()>{
        if !self.items.get(&id).is_none() {
            return Err(anyhow!("item with id {id} is already present"));
        }

        self.items.insert(id, DisjointSet{
            id,
            set_father: id.into(),
            neighbors: Vec::new()
        });

        self.root_items.insert(id);

        return Ok(())
    }

    pub fn set_as_neighbors(&mut self, a: usize, b: usize) -> Result<()>{
        self.items.get_mut(&a).ok_or(anyhow!("unable to find item {a}"))?.neighbors.push(b);
        self.items.get_mut(&b).ok_or(anyhow!("unable to find item {b}"))?.neighbors.push(a);
        Ok(())
    }
}


pub struct DisjointSet{
    id: usize,
    set_father: RefCell<usize>,
    neighbors: Vec<usize>
}

impl DisjointSet {
    pub fn get_father(&self, others: &DisjointSets) -> usize{
        let father = *self.set_father.borrow();
        if self.id == father{
            return self.id;
        }

        let new_father = others.items[&father].get_father(others);
        *self.set_father.borrow_mut() = new_father;

        return new_father
    }
    pub fn set_father(&self, new_father: usize) {
        *self.set_father.borrow_mut() = new_father;
    }
}
