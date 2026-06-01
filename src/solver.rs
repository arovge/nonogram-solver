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

    // entire axis is highlighted
    for i in 0..hints.len() {
        let row = hints.row(i);
        if row.len() == 1 && *row.first().unwrap() == len as u8 {
            set_row(&mut nonogram, i);
        }

        let col = hints.col(i);
        if col.len() == 1 && *col.first().unwrap() == len as u8 {
            set_col(&mut nonogram, i);
        }
    }

    // TODO: This isn't implemented quite right
    // It assumes this strategy is being used on a full axis - not
    // an axis that has become smaller because previous techniques have shortened it.
    //
    // for an axis and a hint:
    // if the axis len == sum of elements + hint len - 1
    // then there is only one possible layout for the hint to fit in the axis
    for i in 0..hints.len() {
        let row = hints.row(i);
        let hint_sum: u8 = row.iter().sum();
        if nonogram.row(i).len() == hint_sum as usize + row.len() - 1 {
            for value in row {
                // nonogram.set(i, col_index);
                // set_row(&mut nonogram, i);
            }
        }

        let col = hints.col(i);
        let hint_sum: u8 = col.iter().sum();
        if nonogram.col(i).len() == hint_sum as usize + col.len() - 1 {
            // set_col(&mut nonogram, i);
        }
    }

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
fn row_len_eq_hint_len_plus_one() {
    let puzzle = NonogramHints::new(
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
