use crate::nonogram::{Cell, NonogramHints, NotSolved, SolvedNonogram, WorkingNonogram};

struct Solver {
    hints: NonogramHints,
    nonogram: WorkingNonogram,
}

impl Solver {
    fn new(hints: NonogramHints) -> Self {
        let nonogram = WorkingNonogram::new(&hints);
        Self { hints, nonogram }
    }

    fn solve(self) -> SolvedNonogram {
        self.try_solve().unwrap()
    }

    fn try_solve(mut self) -> Result<SolvedNonogram, NotSolved> {
        self.check_forcing_full();
        self.check_forcing_with_spaces();

        println!(
            "{:?}",
            SolvedNonogram::from_vec(
                self.nonogram
                    .to_vec()
                    .iter()
                    .map(|r| r.iter().map(|c| matches!(c, Cell::Filled)).collect())
                    .collect()
            )
            .unwrap()
        );

        self.try_evaluate()
    }

    /// Takes the current state of the working nonogram and converts it into a `SolvedNonogram`. Panics if the nonogram is not solved.
    fn evaluate(self) -> SolvedNonogram {
        self.try_evaluate().unwrap()
    }

    /// Takes the current state of the working nonogram and converts it into a `SolvedNonogram`.
    fn try_evaluate(self) -> Result<SolvedNonogram, NotSolved> {
        SolvedNonogram::new(self.nonogram, self.hints)
    }

    /// For a given hint axis of length N,
    /// if the length of the axis is equal to:
    /// - The sum of all values in the hint axis
    /// - Plus the hint len
    /// - Minus 1
    /// then there is only one possible solution for the axis.
    fn check_forcing_with_spaces(&mut self) {
        for i in 0..self.hints.len() {
            let row = self.hints.row(i);
            let hint_sum: u8 = row.iter().sum();
            if !row.is_empty()
                && self
                    .nonogram
                    .row(i)
                    .iter()
                    .filter(|cell| matches!(cell, Cell::Default))
                    .count()
                    == hint_sum as usize + row.len() - 1
            {
                for value in row {
                    for col_index in value as usize..self.len() {
                        self.nonogram.fill(i, col_index);
                    }
                }
            }

            let col = self.hints.col(i);
            let hint_sum: u8 = col.iter().sum();
            if !col.is_empty()
                && self
                    .nonogram
                    .col(i)
                    .iter()
                    .filter(|cell| matches!(cell, Cell::Default))
                    .count()
                    == hint_sum as usize + col.len() - 1
            {
                for value in col {
                    for row_index in value as usize..self.len() {
                        self.nonogram.fill(row_index, i);
                    }
                }
            }
        }
    }

    /// Checks for each row/column in the hint, if the respecting hint
    /// forces the entire row/column to be filled.
    fn check_forcing_full(&mut self) {
        for i in 0..self.hints.len() {
            let row = self.hints.row(i);
            if row.len() == 1 && *row.first().unwrap() == self.len() as u8 {
                self.set_row(i);
            }

            let col = self.hints.col(i);
            if col.len() == 1 && *col.first().unwrap() == self.len() as u8 {
                self.set_col(i);
            }
        }
    }

    fn len(&self) -> usize {
        self.nonogram.len()
    }

    fn set_row(&mut self, index: usize) {
        let len = self.nonogram.len();
        for col_index in 0..len {
            self.nonogram.fill(index, col_index);
        }
    }

    fn set_col(&mut self, index: usize) {
        let len = self.nonogram.len();
        for row_index in 0..len {
            self.nonogram.fill(row_index, index);
        }
    }
}

#[test]
fn forcing_full_col() {
    let hints = NonogramHints::new(
        vec![vec![1], vec![1], vec![1]],
        vec![vec![0], vec![0], vec![3]],
    )
    .unwrap();

    let expected = SolvedNonogram::try_from(
        r#"
        0 0 1
        0 0 1
        0 0 1
    "#,
    )
    .unwrap();

    let result = {
        let mut solver = Solver::new(hints);
        solver.check_forcing_full();
        solver.evaluate()
    };

    assert_eq!(result, expected);
}

#[test]
fn forcing_full_row() {
    let hints = NonogramHints::new(
        vec![vec![3], vec![], vec![3]],
        vec![vec![1, 1], vec![1, 1], vec![1, 1]],
    )
    .unwrap();

    let expected = SolvedNonogram::try_from(
        r#"
        1 1 1
        0 0 0
        1 1 1
    "#,
    )
    .unwrap();

    let result = {
        let mut solver = Solver::new(hints);
        solver.check_forcing_full();
        solver.evaluate()
    };

    assert_eq!(result, expected);
}

#[test]
fn forcing_with_spaces_row() {
    let hints = NonogramHints::new(
        vec![vec![1, 1], vec![], vec![1, 1]],
        vec![vec![1, 1], vec![], vec![1, 1]],
    )
    .unwrap();

    let expected = SolvedNonogram::try_from(
        r#"
        1 0 1
        0 0 0
        1 0 1
    "#,
    )
    .unwrap();

    let result = {
        let mut solver = Solver::new(hints);
        solver.check_forcing_with_spaces();
        solver.evaluate()
    };

    assert_eq!(result, expected);
}

#[test]
fn solves() {
    let hints = NonogramHints::new(
        vec![
            vec![4, 7],
            vec![2, 2, 1, 1],
            vec![2, 3, 1, 1],
            vec![1, 1, 1, 2],
            vec![1, 1, 4],
            vec![2, 1, 4],
            vec![2, 4],
            vec![1, 1, 2],
            vec![2, 2],
            vec![2, 2],
            vec![1, 3],
            vec![2],
            vec![1],
            vec![1],
            vec![1],
        ],
        vec![
            vec![2],
            vec![3],
            vec![1],
            vec![1],
            vec![1],
            vec![0],
            vec![3],
            vec![2, 5],
            vec![3, 3],
            vec![6],
            vec![1],
            vec![12],
            vec![1, 3, 7],
            vec![8, 1],
            vec![1, 5],
        ],
    )
    .unwrap();

    let expected = SolvedNonogram::try_from(
        r#"
        1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
        1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
        1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
        1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
        1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
        1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
        1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
        1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
        1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
        1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
        1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
        1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
        1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
        1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
    "#,
    )
    .unwrap();

    let result = Solver::new(hints).solve();

    assert_eq!(result, expected);
}
