use super::Grid;
use super::Measure;

#[derive(Debug, Clone, Copy)]
pub enum Width {
    Full,
    Half,
    Fourth,
}

impl Measure for Width {
    const MAX_UNITS: u8 = 4;
    fn value(self) -> u8 {
        match self {
            Width::Full => 4,
            Width::Half => 2,
            Width::Fourth => 1,
        }
    }
}

pub fn new<R: Measure + Clone>(gutter: i32) -> Grid<Width, R> {
    Grid {
        gutter,
        columns: vec![],
    }
}
