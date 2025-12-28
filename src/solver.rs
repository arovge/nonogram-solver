use std::iter;

#[derive(Clone)]
struct Nonogram {
    cols: Vec<Vec<u8>>,
    rows: Vec<Vec<u8>>,
}

impl Nonogram {
    fn solve(self) -> SolvedNonogram {
        let len = self.cols.len();

        let cols: Vec<bool> = iter::repeat_n(true, len).collect();
        let mut rows: Vec<Vec<bool>> = iter::repeat_n(cols, len).collect();

        for (index, row) in self.rows.iter().enumerate() {
            if row.len() == 1 && *row.first().unwrap() == 0 {
                rows[index] = iter::repeat_n(false, len).collect();
            }
        }
        for (index, col) in self.cols.iter().enumerate() {
            if col.len() == 1 && *col.first().unwrap() == 0 {
                for row in &mut rows {
                    row[index] = false
                }
            }
        }

        SolvedNonogram { rows }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct SolvedNonogram {
    rows: Vec<Vec<bool>>,
}

impl TryFrom<&str> for SolvedNonogram {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let rows: Vec<&str> = value.trim().split('\n').map(|row| row.trim()).collect();
        let row_count = rows.len();
        assert_eq!(row_count, 15);

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
                        if *value == "0" { false } else { true }
                    })
                    .collect()
            })
            .collect();

        let solved = SolvedNonogram { rows };

        Ok(solved)
    }
}

#[test]
fn solves() {
    let puzzle = Nonogram {
        cols: vec![
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
        rows: vec![
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
    };

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

    for (index, row) in puzzle.clone().solve().rows.iter().enumerate() {
        assert_eq!(row.len(), 15);
        assert_eq!(*row, expected.rows[index]);
    }

    //assert_eq!(puzzle.solve(), expected);
}
