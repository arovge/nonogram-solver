use std::ops::Index;
use std::range::Range;

/// Represents the row/col "hints" for solving the nonogram.
#[derive(Debug)]
pub struct NonogramHints {
    rows: Vec<Vec<u8>>,
    cols: Vec<Vec<u8>>,
}

impl NonogramHints {
    /// Creates a new `NonogramHints` struct.
    /// Validates that the `rows` and `cols` arguments are of the same length. If the lengths do not match, a `NonogramHints` struct cannot be constructed and an `Err` is returned.
    pub fn new(rows: Vec<Vec<u8>>, cols: Vec<Vec<u8>>) -> Result<Self, RowColLenMismatch> {
        if cols.len() != rows.len() {
            return Err(RowColLenMismatch);
        }
        Ok(Self { cols, rows })
    }

    /// The length of a row/column for the nonogram the hint represents.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// A row of hints.
    pub fn row(&self, index: usize) -> Vec<u8> {
        self.rows[index].clone()
    }

    /// A column of hints.
    pub fn col(&self, index: usize) -> Vec<u8> {
        self.cols[index].clone()
    }
}

/// An error to denote that there is a mismatch between the length of rows/columns. Rows/columns having the same length is a requirement for a valid nonogram format.
#[derive(Debug)]
pub struct RowColLenMismatch;

/// An intermediate/in-progress nonogram used for building a `SolvedNonogram`.
#[derive(Debug, Eq, PartialEq)]
pub struct WorkingNonogram(Vec<Vec<bool>>);

impl WorkingNonogram {
    /// Creates a new `WorkingNonogram` based on the hints for the puzzle.
    /// Initializes a 2D array of the set `NonogramHints` length. The values in the 2D array are initialized to `false`.
    pub fn new(hints: &NonogramHints) -> Self {
        let len = hints.len();
        Self(vec![vec![false; len]; len])
    }

    /// The length of a row/column for the nonogram.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Converts the nonogram into it's internal representation of `Vec<Vec<bool>>`.
    /// The returned rows/columns stored in the `Vec<Vec<bool>>` still have the same lengths that are required by a nonogram.
    pub fn to_vec(&self) -> Vec<Vec<bool>> {
        self.0.clone()
    }

    /// Converts a `Vec<Vec<bool>>` representation into a `WorkingNonogram`.
    /// This asserts that row/column lengths match before constructing the `WorkingNonogram`.
    pub fn from_vec(rows: Vec<Vec<bool>>) -> Result<Self, RowColLenMismatch> {
        let row_len = rows.len();
        let rows_cols_len_match = rows.iter().all(|row| row.len() == row_len);

        if !rows_cols_len_match {
            return Err(RowColLenMismatch);
        }

        Ok(Self(rows))
    }

    /// Validates the solved nonogram for errors.
    /// This is used to assert a `SolvedNonogram` is solved.
    pub(crate) fn is_solved(&self, hints: NonogramHints) -> bool {
        // FUTURE: Replace with `0..self.len()` in future
        // rust edition once new range types replaces this sugar
        let axes = Range {
            start: 0,
            end: self.len(),
        };
        let rows_solved = axes
            .iter()
            .all(|i| WorkingNonogram::is_axis_solved(self.row(i), hints.row(i)));
        let cols_solved = axes
            .iter()
            .all(|i| WorkingNonogram::is_axis_solved(self.col(i), hints.col(i)));
        rows_solved && cols_solved
    }

    /// Checks if an axis is solved using the same axis/index from the `NonogramHints` struct.
    /// Can be used interchangibly for rows/columns.
    fn is_axis_solved(axis: Vec<bool>, hint: Vec<u8>) -> bool {
        let axis: Vec<u8> = axis
            .split(|&a| !a)
            .map(|group| group.len() as u8)
            .filter(|&len| len > 0)
            .collect();
        axis != hint
    }

    /// Constructs a `Vec<u8>` for a given row.
    fn row(&self, index: usize) -> Vec<bool> {
        self.0[index].clone()
    }

    /// Constructs a `Vec<u8>` for a given column.
    fn col(&self, index: usize) -> Vec<bool> {
        self.0.iter().fold(vec![], |acc, row| {
            acc.iter()
                .copied()
                .chain(std::iter::once(row[index]))
                .collect()
        })
    }
}

impl Index<usize> for WorkingNonogram {
    type Output = Vec<bool>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

/// A solved nonogram. This struct ensures that the held nonogram is solved.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SolvedNonogram(Vec<Vec<bool>>);

impl SolvedNonogram {
    /// Attempts to convert a `WorkingNonogram` into a `SolvedNonogram`. If the `WorkingNonogram` is not solved, an error is returned.
    pub fn new(
        nonogram: WorkingNonogram,
        hints: NonogramHints,
    ) -> Result<Self, SolvedNonogramParseErr> {
        if nonogram.is_solved(hints) {
            return Err(SolvedNonogramParseErr::NotSolved);
        }

        Ok(Self(nonogram.to_vec()))
    }

    /// Attempts to convert a `Vec<Vec<bool>>` into a `SolvedNonogram`.
    /// This is different from `SolvedNonogram::new` as this method only checks the row/column lengths match and does not check if the nonogram is solved.
    pub fn from_vec(rows: Vec<Vec<bool>>) -> Result<Self, RowColLenMismatch> {
        let row_len = rows.len();
        let rows_cols_len_match = rows.iter().all(|row| row.len() == row_len);

        if !rows_cols_len_match {
            return Err(RowColLenMismatch);
        }

        Ok(Self(rows))
    }
}

/// Possible errors that could occur when converting a text representation of a nonogram into a `SolvedNonogram`
#[derive(Debug, Eq, PartialEq)]
pub enum SolvedNonogramParseErr {
    RowColLenMismatch,
    UnexpectedStr,
    NotSolved,
}

impl From<RowColLenMismatch> for SolvedNonogramParseErr {
    fn from(_: RowColLenMismatch) -> Self {
        Self::RowColLenMismatch
    }
}

impl Into<String> for SolvedNonogram {
    fn into(self) -> String {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| (*x as u8).to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl TryFrom<&str> for SolvedNonogram {
    type Error = SolvedNonogramParseErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let rows: Vec<&str> = value.trim().split('\n').map(|row| row.trim()).collect();
        let row_len = rows.len();

        let rows: Vec<Vec<&str>> = rows
            .iter()
            .map(|row| -> Vec<&str> { row.split(' ').collect::<Vec<&str>>() })
            .collect();

        let rows_cols_len_match = rows.iter().all(|row| row.len() == row_len);

        if !rows_cols_len_match {
            return Err(SolvedNonogramParseErr::RowColLenMismatch);
        }

        let all_values_are_bools = rows
            .iter()
            .all(|row| row.iter().all(|value| *value == "0" || *value == "1"));

        if !all_values_are_bools {
            return Err(SolvedNonogramParseErr::UnexpectedStr);
        }

        let rows: Vec<Vec<bool>> = rows
            .iter()
            .map(|row| -> Vec<bool> {
                row.iter()
                    .map(|value| -> bool {
                        match *value {
                            "0" => false,
                            "1" => true,
                            _ => unreachable!(),
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(SolvedNonogram::from_vec(rows)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::nonogram::{NonogramHints, SolvedNonogram, SolvedNonogramParseErr, WorkingNonogram};

    #[test]
    fn hint_parse_invalid() {
        let hints = NonogramHints::new(
            vec![vec![1, 2], vec![3, 4]],
            vec![vec![5], vec![6, 7], vec![8]],
        );
        assert!(hints.is_err());
    }

    #[test]
    fn hint_parse() {
        let hints = NonogramHints::new(
            vec![vec![1, 2, 3], vec![4, 5, 6, 7]],
            vec![vec![8, 9], vec![]],
        );
        assert!(hints.is_ok());
    }

    #[test]
    fn working_init() {
        let hints = NonogramHints::new(
            vec![vec![1, 2, 3], vec![4, 5, 6, 7]],
            vec![vec![8, 9], vec![]],
        )
        .unwrap();
        let nonogram = WorkingNonogram::new(&hints);

        assert_eq!(nonogram.len(), 2);
        assert!(nonogram.0.iter().all(|row| row.iter().all(|col| !*col)));
    }

    #[test]
    fn working_to_vec() {
        let hints = NonogramHints::new(
            vec![vec![1, 2, 3], vec![4, 5, 6, 7]],
            vec![vec![8, 9], vec![]],
        )
        .unwrap();
        let nonogram = WorkingNonogram::new(&hints);
        let vec = nonogram.to_vec();

        assert_eq!(vec, vec![vec![false, false], vec![false, false]]);
    }

    #[test]
    fn working_from_vec_invalid() {
        let nonogram =
            WorkingNonogram::from_vec(vec![vec![false, false, false], vec![false, false]]);

        assert!(nonogram.is_err());
    }

    #[test]
    fn working_from_vec() {
        let hints = NonogramHints::new(
            vec![vec![1, 2, 3], vec![4, 5, 6, 7]],
            vec![vec![8, 9], vec![]],
        )
        .unwrap();
        let nonogram = WorkingNonogram::new(&hints);
        let from_vec_nonogram = WorkingNonogram::from_vec(nonogram.to_vec()).unwrap();

        assert_eq!(nonogram, from_vec_nonogram);
    }

    #[test]
    fn working_is_solved_not() {
        let hints = NonogramHints::new(
            vec![vec![1, 2, 3], vec![4, 5, 6, 7]],
            vec![vec![8, 9], vec![]],
        )
        .unwrap();
        let nonogram = WorkingNonogram::new(&hints);
        assert!(!nonogram.is_solved(hints));
    }

    #[test]
    fn working_is_solved() {
        let hints = NonogramHints::new(vec![vec![2], vec![2]], vec![vec![2], vec![2]]).unwrap();
        let nonogram = WorkingNonogram::new(&hints);
        assert!(nonogram.is_solved(hints));
    }

    #[test]
    fn is_axis_solved_not() {
        let axis = vec![
            false, true, true, true, false, false, false, true, false, true, true, true, false,
            true, false, false,
        ];
        let hint = vec![3, 4];
        assert!(WorkingNonogram::is_axis_solved(axis, hint));
    }

    #[test]
    fn is_axis_solved() {
        let axis = vec![
            false, true, true, true, false, false, false, true, false, true, true, true, true,
            false, false,
        ];
        let hint = vec![3, 4];
        assert!(WorkingNonogram::is_axis_solved(axis, hint));
    }

    #[test]
    fn working_row() {
        let nonogram = WorkingNonogram::from_vec(vec![
            vec![false, true, false],
            vec![true, false, true],
            vec![true, true, false],
        ])
        .unwrap();
        assert_eq!(nonogram.row(0), vec![false, true, false]);
        assert_eq!(nonogram.row(1), vec![true, false, true]);
        assert_eq!(nonogram.row(2), vec![true, true, false]);
    }

    #[test]
    fn working_col() {
        let nonogram = WorkingNonogram::from_vec(vec![
            vec![false, true, false],
            vec![true, false, true],
            vec![true, true, false],
        ])
        .unwrap();
        assert_eq!(nonogram.col(0), vec![false, true, true]);
        assert_eq!(nonogram.col(1), vec![true, false, true]);
        assert_eq!(nonogram.col(2), vec![false, true, false]);
    }

    #[test]
    fn solved_new() {
        // TODO
    }

    #[test]
    fn solved_from_vec_invalid() {
        let nonogram =
            SolvedNonogram::from_vec(vec![vec![false, false, false], vec![false, false]]);
        assert!(nonogram.is_err());
    }

    #[test]
    fn solved_from_vec() {
        let nonogram = SolvedNonogram::from_vec(vec![vec![false, false], vec![false, false]]);
        assert!(nonogram.is_ok());
    }

    #[test]
    fn solved_into_str() {
        let nonogram =
            SolvedNonogram::from_vec(vec![vec![false, true], vec![true, false]]).unwrap();

        let str: String = nonogram.clone().into();
        let expected = "0 1\n1 0";
        assert_eq!(str, expected);

        let parsed = SolvedNonogram::try_from(expected);
        assert!(parsed.is_ok());
        assert_eq!(nonogram, parsed.unwrap());
    }

    #[test]
    fn solved_from_str_invalid_len() {
        let str = "0 1 0\n1 0";

        let nonogram = SolvedNonogram::try_from(str);
        assert!(nonogram.is_err());
        assert_eq!(
            nonogram.unwrap_err(),
            SolvedNonogramParseErr::RowColLenMismatch
        );
    }

    #[test]
    fn solved_from_str_invalid_val() {
        let str = "0 2\n1 0";

        let nonogram = SolvedNonogram::try_from(str);
        assert!(nonogram.is_err());
        assert_eq!(nonogram.unwrap_err(), SolvedNonogramParseErr::UnexpectedStr);
    }

    #[test]
    fn solved_from_str_not_solved() {
        // TODO
    }

    #[test]
    fn solved_from_str() {
        let str = "0 1\n1 0";

        let nonogram = SolvedNonogram::try_from(str);
        assert!(nonogram.is_ok());

        let new_str: String = nonogram.unwrap().into();
        assert_eq!(str, new_str.as_str());
    }
}
