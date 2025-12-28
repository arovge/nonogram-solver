#[derive(Clone)]
pub struct Nonogram {
    pub cols: Vec<Vec<u8>>,
    pub rows: Vec<Vec<u8>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct SolvedNonogram {
    pub rows: Vec<Vec<bool>>,
}

impl TryFrom<&str> for SolvedNonogram {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let rows: Vec<&str> = value.trim().split('\n').map(|row| row.trim()).collect();
        let row_count = rows.len();

        let rows: Vec<Vec<&str>> = rows
            .iter()
            .map(|row| -> Vec<&str> { row.split(' ').collect::<Vec<&str>>() })
            .collect();

        let cols_match_row_count = rows.iter().all(|row| row.len() == row_count);

        if !cols_match_row_count {
            return Err("Not all columns match row count");
        }

        let all_values_are_bools = rows
            .iter()
            .all(|row| row.iter().all(|value| *value == "0" || *value == "1"));

        if !all_values_are_bools {
            return Err("Not all values are bools");
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

        let solved = SolvedNonogram { rows };

        Ok(solved)
    }
}
