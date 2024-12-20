use crate::prelude::*;

pub enum AreaMarker{
    SquaredArea{height: i32, width: i32, y_start: i32, y_end: i32},
    MaskedAre(Mat)
}

pub struct Area{
    id: usize,
    marker: AreaMarker
}



pub struct ImageLogger{
}
