/// Represents the row/col "hints" for solving the nonogram.
#[derive(Debug)]
pub struct NonogramHints {
    rows: Vec<Vec<u8>>,
    cols: Vec<Vec<u8>>,
}

impl NonogramHints {
    pub fn rows(&self) -> Vec<Vec<u8>> {
        self.rows.clone()
    }

    pub fn cols(&self) -> Vec<Vec<u8>> {
        self.cols.clone()
    }

    pub fn new(rows: Vec<Vec<u8>>, cols: Vec<Vec<u8>>) -> Result<Self, NonogramHintsParseErr> {
        // TODO: More validation for more lens
        if cols.len() != rows.len() {
            return Err(NonogramHintsParseErr::RowColLenMismatch);
        }
        Ok(Self { cols, rows })
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

#[derive(Debug)]
pub enum NonogramHintsParseErr {
    RowColLenMismatch,
}

pub struct WorkingNonogram(Vec<Vec<bool>>);

impl WorkingNonogram {
    pub fn new(len: usize) -> Self {
        Self(vec![vec![false; len]; len])
    }

    pub fn rows(&self) -> Vec<Vec<bool>> {
        self.0.clone()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SolvedNonogram(Vec<Vec<bool>>);

impl SolvedNonogram {
    pub fn new(rows: Vec<Vec<bool>>) -> Result<Self, SolvedNonogramParseEr> {
        let nonogram = Self::new_no_validate(rows)?;

        nonogram
            .validate()
            .map_err(|_| SolvedNonogramParseEr::NotSolved)?;

        Ok(nonogram)
    }

    pub fn new_no_validate(rows: Vec<Vec<bool>>) -> Result<Self, SolvedNonogramParseEr> {
        let row_count = rows.len();
        let col_row_len_match = rows.iter().all(|row| row.len() == row_count);

        if !col_row_len_match {
            return Err(SolvedNonogramParseEr::RowColLenMismatch);
        }

        Ok(Self(rows))
    }

    /// Validates the solved nonogram for errors.
    /// This is used to assert a `SolvedNonogram` is in fact solved.
    fn validate(&self) -> Result<(), ()> {
        Ok(())
    }
}

#[derive(Debug)]
pub enum SolvedNonogramParseEr {
    RowColLenMismatch,
    UnexpectedStr,
    NotSolved,
}

impl TryFrom<&str> for SolvedNonogram {
    type Error = SolvedNonogramParseEr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let rows: Vec<&str> = value.trim().split('\n').map(|row| row.trim()).collect();
        let row_count = rows.len();

        let rows: Vec<Vec<&str>> = rows
            .iter()
            .map(|row| -> Vec<&str> { row.split(' ').collect::<Vec<&str>>() })
            .collect();

        let cols_match_row_count = rows.iter().all(|row| row.len() == row_count);

        if !cols_match_row_count {
            return Err(Self::Error::RowColLenMismatch);
        }

        let all_values_are_bools = rows
            .iter()
            .all(|row| row.iter().all(|value| *value == "0" || *value == "1"));

        if !all_values_are_bools {
            return Err(Self::Error::UnexpectedStr);
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

        let nonogram = SolvedNonogram::new(rows).unwrap();

        Ok(nonogram)
    }
}
