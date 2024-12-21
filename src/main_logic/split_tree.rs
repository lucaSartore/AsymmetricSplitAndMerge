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

    pub fn collect_leafs(&'a self, others: &'a [SplitTree<'a>], accumulator: &mut Vec<(usize,&ImageContainerSplit<'a>)>){
        if let Some([a,b]) = self.childs{
            others[a].collect_leafs(others, accumulator);
            others[b].collect_leafs(others, accumulator);
        }else{
            accumulator.push((self.id, &self.image));
        }
    }
}
