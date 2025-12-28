use crate::nonogram::{Nonogram, SolvedNonogram};
use std::iter;

fn solve(nonogram: Nonogram) -> SolvedNonogram {
    let len = nonogram.cols.len();

    let cols: Vec<bool> = iter::repeat_n(true, len).collect();
    let mut rows: Vec<Vec<bool>> = iter::repeat_n(cols, len).collect();

    fn set_row(rows: &mut Vec<Vec<bool>>, index: usize, value: bool) {
        rows[index] = iter::repeat_n(value, rows.len()).collect();
    }

    fn set_col(rows: &mut Vec<Vec<bool>>, index: usize, value: bool) {
        for row in rows {
            row[index] = value
        }
    }

    for (index, row) in nonogram.rows.iter().enumerate() {
        if row.len() == 1 {
            if *row.first().unwrap() == 0 {
                set_row(&mut rows, index, false);
            } else if *row.first().unwrap() == len as u8 {
                set_row(&mut rows, index, true);
            }
        }
    }
    for (index, col) in nonogram.cols.iter().enumerate() {
        if col.len() == 1 {
            if *col.first().unwrap() == 0 {
                set_col(&mut rows, index, false);
            } else if *col.first().unwrap() == len as u8 {
                set_col(&mut rows, index, true);
            }
        }
    }

    SolvedNonogram { rows }
}

#[test]
fn full_col() {
    let puzzle = Nonogram {
        cols: vec![vec![3], vec![0], vec![3]],
        rows: vec![vec![1, 1], vec![1, 1], vec![1, 1]],
    };

    let expected = SolvedNonogram::try_from(
        r#"
        1 0 1
        1 0 1
        1 0 1
    "#,
    )
    .unwrap();

    assert_eq!(solve(puzzle), expected);
}

#[test]
fn full_row() {
    let puzzle = Nonogram {
        cols: vec![vec![1, 1], vec![1, 1], vec![1, 1]],
        rows: vec![vec![3], vec![0], vec![3]],
    };

    let expected = SolvedNonogram::try_from(
        r#"
        1 1 1
        0 0 0
        1 1 1
    "#,
    )
    .unwrap();

    assert_eq!(solve(puzzle), expected);
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

    assert_eq!(solve(puzzle), expected);
}
