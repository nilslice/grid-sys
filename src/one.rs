use super::Grid;
use super::Measure;

#[derive(Debug, Clone, Copy)]
pub enum Width {
    Full,
}

impl Measure for Width {
    const MAX_UNITS: u8 = 1;

    fn value(self) -> u8 {
        1
    }
}

pub fn new<R: Measure + Clone>(spacing: i32) -> Grid<Width, R> {
    Grid {
        spacing,
        columns: vec![],
    }
}
