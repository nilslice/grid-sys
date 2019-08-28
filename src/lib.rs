mod eight;
mod four;
mod one;

pub trait Measure {
    const MAX_UNITS: u8;
    fn value(self) -> u8;
}

#[derive(Debug)]
pub struct Grid<C, R>
where
    C: Measure + Clone,
    R: Measure + Clone,
{
    spacing: i32,
    columns: Vec<Column<C, R>>,
}

impl<C, R> Grid<C, R>
where
    C: Measure + Clone,
    R: Measure + Clone,
{
    pub fn add_column(&mut self, col: Column<C, R>) {
        self.columns.push(col);
    }

    pub fn get_overflow(&mut self) -> Vec<Column<C, R>> {
        let mut val = 0u8;
        for (i, col) in self.columns.iter().enumerate() {
            if val >= C::MAX_UNITS {
                return self.columns.drain(i..).collect();
            }

            val += col.get_width().value();
        }

        vec![]
    }
}

#[derive(Debug)]
pub struct Column<C, R>
where
    C: Measure + Clone,
    R: Measure + Clone,
{
    width: C,
    rows: Vec<Row<R>>,
}

impl<C, R> Column<C, R>
where
    C: Measure + Clone,
    R: Measure + Clone,
{
    fn of(width: C) -> Column<C, R> {
        Column {
            width,
            rows: vec![],
        }
    }

    fn get_width(&self) -> C {
        self.width.clone()
    }

    fn add_row(&mut self, row: Row<R>) {
        self.rows.push(row);
    }
}

#[derive(Debug)]
pub struct Row<R> {
    width: R,
}

#[cfg(test)]
mod tests {
    #[test]
    fn fits_eight() {
        use super::eight;
        use super::one;
        use super::Column;
        use super::Row;

        let mut grid = eight::new(20);

        grid.add_column(Column::of(eight::Width::Half));
        grid.add_column(Column::of(eight::Width::Fourth));
        let mut c = Column::of(eight::Width::Eigth);
        c.add_row(Row {
            width: one::Width::Full,
        });
        grid.add_column(c);
        grid.add_column(Column::of(eight::Width::Eigth));
        println!("8 {:?}", grid);

        assert_eq!(grid.get_overflow().len(), 0);
    }

    #[test]
    fn overflows_one() {
        use super::one;
        use super::Column;
        use super::Grid;

        let mut grid: Grid<one::Width, one::Width> = one::new(5);
        println!("1 {:?}", grid);

        // intentionally overflow the grid with columns, using 2 full width
        // columns. this should reult in a single length overflow vec.
        grid.add_column(Column {
            width: one::Width::Full,
            rows: vec![],
        });
        grid.add_column(Column {
            width: one::Width::Full,
            rows: vec![],
        });
        println!("overflowing grid = {:?}", grid);

        let overflow = grid.get_overflow();
        println!("overflow = {:?}", overflow);
        assert_eq!(overflow.len(), 1);
    }
}
