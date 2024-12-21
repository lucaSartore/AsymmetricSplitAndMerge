use super::*;

pub struct SplitTree<'a> {
    pub id: usize,
    pub image: ImageContainerSplit<'a>,
    pub childs: Option<[usize; 2]>,
}

impl<'a> SplitTree<'a> {
    pub fn new(id: usize, image: ImageContainerSplit<'a>) -> Self{
        return Self{
            id,
            image,
            childs: None
        };
    } 
}
