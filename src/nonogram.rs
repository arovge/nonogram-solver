use std::ops::Index;

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
    pub fn rows(&self) -> Vec<Vec<u8>> {
        self.rows.clone()
    }

    /// A column of hints.
    pub fn cols(&self) -> Vec<Vec<u8>> {
        self.cols.clone()
    }
}

/// An error to denote that there is a mismatch between the length of rows/columns. Rows/columns having the same length is a requirement for a valid nonogram format.
#[derive(Debug)]
pub struct RowColLenMismatch;

/// An intermediate/in-progress nonogram used for building a `SolvedNonogram`.
pub struct WorkingNonogram(Vec<Vec<bool>>);

impl WorkingNonogram {
    /// Creates a new `WorkingNonogram` based on the hints for the puzzle.
    /// Initializes a 2D array of the set `NonogramHints` length. The values in the 2D array are initialized to `false`.
    pub fn new(hints: NonogramHints) -> Self {
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
}

impl Index<usize> for WorkingNonogram {
    type Output = Vec<bool>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

/// A solved nonogram. This struct ensures that the held nonogram is solved.
#[derive(Debug, Eq, PartialEq)]
pub struct SolvedNonogram(Vec<Vec<bool>>);

impl SolvedNonogram {
    /// Attempts to convert a `WorkingNonogram` into a `SolvedNonogram`. If the `WorkingNonogram` is not solved, an error is returned.
    pub fn new(nonogram: WorkingNonogram) -> Result<Self, SolvedNonogramParseErr> {
        let nonogram = Self(nonogram.to_vec());

        if nonogram.is_solved() {
            return Err(SolvedNonogramParseErr::NotSolved);
        }

        Ok(nonogram)
    }

    /// Convert a `WorkingNonogram` into a `SolvedNonogram`, ignoring whether the `WorkingNonogram` is solved or not.
    pub fn new_no_validate(nonogram: WorkingNonogram) -> Self {
        Self(nonogram.to_vec())
    }

    /// Attempts to convert a `Vec<Vec<bool>>` into a `SolvedNonogram`.
    /// This is different from `SolvedNonogram::new` as this method checks the row/column lengths match, in addition to if the nonogram is solved. The former would have been checked by passing a `WorkingNonogram` struct to `SolvedNonogram::new`.
    pub fn from_vec(rows: Vec<Vec<bool>>) -> Result<Self, SolvedNonogramParseErr> {
        SolvedNonogram::new(WorkingNonogram::from_vec(rows)?)
    }

    /// Validates the solved nonogram for errors.
    /// This is used to assert a `SolvedNonogram` is in fact solved.
    fn is_solved(&self) -> bool {
        // TODO: the rest of the owl
        true
    }
}

/// Possible errors that could occur when converting a text representation of a nonogram into a `SolvedNonogram`
#[derive(Debug)]
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

        SolvedNonogram::from_vec(rows)
    }
}
