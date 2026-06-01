use crate::nonogram::{NonogramHints, NotSolved, SolvedNonogram, WorkingNonogram};
use std::iter;

fn solve(hints: NonogramHints) -> Result<SolvedNonogram, NotSolved> {
    let mut nonogram = WorkingNonogram::new(&hints);
    let len = nonogram.len();

    fn set_row(nonogram: &mut WorkingNonogram, index: usize) {
        let len = (*nonogram).len();
        for col_index in 0..len {
            nonogram.set(index, col_index);
        }
    }

    fn set_col(nonogram: &mut WorkingNonogram, index: usize) {
        let len = (*nonogram).len();
        for row_index in 0..len {
            nonogram.set(row_index, index);
        }
    }

    for i in 0..hints.len() {
        let row = hints.row(i);
        if row.len() == 1 && row.first().is_some_and(|a| *a == len as u8) {
            set_row(&mut nonogram, i);
        }

        let col = hints.col(i);
        if col.len() == 1 && col.first().is_some_and(|a| *a == len as u8) {
            set_col(&mut nonogram, i);
        }
    }

    // for (index, row) in nonogram.rows().iter().enumerate() {
    //     if row.len() == 1 {
    //         if *row.first().unwrap() == false {
    //             set_row(&mut rows, index, false);
    //         }
    // else if *row.first().unwrap() == len as u8 {
    //     set_row(&mut rows, index, true);
    // }
    //     }
    // }
    // for (index, col) in nonogram.cols().iter().enumerate() {
    //     if col.len() == 1 {
    //         if *col.first().unwrap() == 0 {
    //             set_col(&mut rows, index, false);
    //         } else if *col.first().unwrap() == len as u8 {
    //             set_col(&mut rows, index, true);
    //         }
    //     }
    // }

    SolvedNonogram::new(nonogram, hints)
}

#[test]
fn full_col() {
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

    assert_eq!(solve(hints).ok(), Some(expected));
}

#[test]
fn full_row() {
    let puzzle = NonogramHints::new(
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

    assert_eq!(solve(puzzle).ok(), Some(expected));
}

#[test]
fn solves() {
    let puzzle = NonogramHints::new(
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

    assert_eq!(solve(puzzle).ok(), Some(expected));
}
