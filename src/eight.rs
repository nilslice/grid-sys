use super::Grid;
use super::Measure;

#[derive(Debug, Clone, Copy)]
pub enum Width {
    Full,
    Half,
    Fourth,
    Eigth,
}

impl Measure for Width {
    const MAX_UNITS: u8 = 8;

    fn value(self) -> u8 {
        match self {
            Width::Full => 8,
            Width::Half => 4,
            Width::Fourth => 2,
            Width::Eigth => 1,
        }
    }
}

pub fn new<R: Measure + Clone>(gutter: i32) -> Grid<Width, R> {
    Grid {
        gutter,
        columns: vec![],
    }
}
