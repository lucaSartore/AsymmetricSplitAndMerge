use crate::prelude::*;
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::{hash_set::Iter, HashMap, HashSet},
};

#[derive(Debug, Default)]
pub struct DisjointSets {
    items: HashMap<usize, DisjointSet>,
    root_items: HashSet<usize>,
}

impl DisjointSets {
    pub fn get_root_items(&self) -> Iter<'_, usize> {
        return self.root_items.iter();
    }

    pub fn get_tuple_of_items_to_check(&self) -> Vec<[usize; 2]> {
        let mut already_present_items = HashSet::<usize>::default();
        let mut to_return = Vec::new();

        for a in self.root_items.iter() {
            if already_present_items.contains(a) {
                continue;
            }
            for b in self.items[a].inner.borrow_mut().neighbors.iter() {
                if already_present_items.contains(b) {
                    continue;
                }
                already_present_items.insert(*a);
                already_present_items.insert(*b);
                to_return.push([*a,*b]);
                // ad this point a has been insert, therefore no iteration of this for loop
                // will be considered valid
                break; 
            }
        }
        return to_return;
    }

    pub fn get_set(&self, id: usize) -> Option<&DisjointSet> {
        return self.items.get(&id);
    }

    pub fn get_father_of(&self, id: usize) -> Option<usize> {
        return Some(self.items.get(&id)?.get_father(self));
    }

    pub fn is_root_item(&self, id: usize) -> bool {
        return self.root_items.contains(&id);
    }

    pub fn mark_as_non_neighbors(&self, id1: usize, id2: usize) -> Result<()> {
        self.items
            .get(&id1)
            .ok_or(anyhow!("item 1 not found"))?
            .inner
            .borrow_mut()
            .neighbors
            .remove(&id2);

        self.items
            .get(&id2)
            .ok_or(anyhow!("item 2 not found"))?
            .inner
            .borrow_mut()
            .neighbors
            .remove(&id1);
        Ok(())
    }

    pub fn create_new(&mut self, new_item_id: usize, childrends: [usize; 2]) -> Result<()> {
        let [c1, c2] = childrends;

        let c1 = self
            .items
            .get(&c1)
            .ok_or(anyhow!("unable to find the id {c1}"))?
            .get_father(self);
        let c2 = self
            .items
            .get(&c2)
            .ok_or(anyhow!("unable to find the id {c2}"))?
            .get_father(self);

        self.items[&c1].set_father(new_item_id);
        self.items[&c2].set_father(new_item_id);

        self.items.insert(
            new_item_id,
            DisjointSet {
                id: new_item_id,
                inner: RefCell::new(DisjointSetInner {
                    father: new_item_id,
                    neighbors: HashSet::new(),
                }),
            },
        );

        let neighbors = self.items[&c1]
            .inner
            .borrow()
            .neighbors
            .iter()
            .chain(self.items[&c2].inner.borrow().neighbors.iter())
            .map(|x| *x)
            .collect();

        self.items
            .get(&new_item_id)
            .expect("error in code of disjoint set building")
            .inner
            .borrow_mut()
            .neighbors = neighbors;

        self.root_items.remove(&c1);
        self.root_items.remove(&c2);
        self.root_items.insert(new_item_id);
        Ok(())
    }

    pub fn add_item(&mut self, id: usize) -> Result<()> {
        if !self.items.get(&id).is_none() {
            return Err(anyhow!("item with id {id} is already present"));
        }

        self.items.insert(
            id,
            DisjointSet {
                id,
                inner: DisjointSetInner {
                    father: id,
                    neighbors: HashSet::new(),
                }
                .into(),
            },
        );

        self.root_items.insert(id);

        return Ok(());
    }

    pub fn set_as_neighbors(&mut self, a: usize, b: usize) -> Result<()> {
        self.items
            .get_mut(&a)
            .ok_or(anyhow!("unable to find item {a}"))?
            .inner
            .borrow_mut()
            .neighbors
            .insert(b);
        self.items
            .get_mut(&b)
            .ok_or(anyhow!("unable to find item {b}"))?
            .inner
            .borrow_mut()
            .neighbors
            .insert(a);
        Ok(())
    }

    pub fn clear_data(&self) {
        self.root_items.iter().for_each(|x| {
            let new_neighbors = self.items[x]
                .inner
                .borrow()
                .neighbors
                .iter()
                .map(|y| {
                    self.get_father_of(*y) // take only the father
                        .expect("error while cleaning data")
                })
                .filter(|z| {
                    *z != self.items[x].id // remove reference to self
                })
                .collect();
            self.items[x].inner.borrow_mut().neighbors = new_neighbors;
        });
    }
}

#[derive(Debug)]
struct DisjointSetInner {
    father: usize,
    neighbors: HashSet<usize>,
}
#[derive(Debug)]
pub struct DisjointSet {
    id: usize,
    inner: RefCell<DisjointSetInner>,
}

impl DisjointSet {
    pub fn get_father(&self, others: &DisjointSets) -> usize {
        let father = self.inner.borrow().father;
        if self.id == father {
            return self.id;
        }

        let new_father = others.items[&father].get_father(others);
        self.set_father(new_father);
        return new_father;
    }
    pub fn set_father(&self, new_father: usize) {
        self.inner.borrow_mut().father = new_father;
    }
}
