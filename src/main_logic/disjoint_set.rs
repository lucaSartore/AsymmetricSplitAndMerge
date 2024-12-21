use std::{cell::RefCell, collections::{hash_set::Iter, HashMap, HashSet}};
use crate::prelude::*;


#[derive(Debug,Default)]
pub struct DisjointSets{
    items: HashMap<usize, DisjointSet>,
    root_items: HashSet<usize>
}

impl DisjointSets{

    pub fn get_root_items(&self) -> Iter<'_,usize>{
        return self.root_items.iter()
    }

    pub fn get_set(&self, id: usize) -> Option<&DisjointSet>{
        return self.items.get(&id);
    }

    pub fn get_father_of(&self, id: usize) -> Option<usize> {
        return Some(self.items.get(&id)?.get_father(self))
    }

    pub fn is_root_item(&self, id: usize) -> bool {
        return self.root_items.contains(&id);
    }

    pub fn create_new(&mut self, new_item_id: usize, childrends: [usize;2]) -> Result<()> {

        let [c1,c2] = childrends; 

        let c1 = self.items.get(&c1)
            .ok_or(anyhow!("unable to find the id {c1}"))?
            .get_father(self);
        let c2 = self.items.get(&c2)
            .ok_or(anyhow!("unable to find the id {c2}"))?
            .get_father(self);
        
        self.items[&c1].set_father(new_item_id);
        self.items[&c2].set_father(new_item_id);
        
        self.items.insert(new_item_id, DisjointSet{
            id: new_item_id,
            set_father: RefCell::new(new_item_id),
            neighbors: Vec::new()
        });

        let neighbors: Vec<_> = self.items[&c1].neighbors.iter()
            .chain(self.items[&c2].neighbors.iter())
            .filter(|x| {
                self.items[&x].get_father(self) != new_item_id
            })
            .map(|x| *x)
            .collect::<HashSet<_>>() // unique ids
            .into_iter()
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


#[derive(Debug)]
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
    pub fn get_neighbors(&self) -> &[usize]{
        return &self.neighbors;
    }
}
