#![cfg(test)]

use super::*;
use indoc::indoc;
use pretty_assertions::assert_eq;

#[test]
fn display() {
    let board =
        HexagonalBoard::from([([0, 0], 'a'), ([1, 0], 'b'), ([0, 1], 'c'), ([-1, -1], 'd')]);

    let output = format!("{board}");
    let expected = indoc!(
        r"
             /---\     /---\
            ⟨  b  ⟩---⟨  c  ⟩
             \---⟨  a  ⟩---/
                  ⟩---⟨
                 ⟨  d  ⟩
                  \---/
        "
    );

    println!("{output}");
    println!("{expected}");

    for (output, expected) in output.split('\n').zip(expected.split('\n')) {
        for (output, expected) in expected.chars().zip(output.chars()) {
            assert_eq!(output, expected)
        }
    }
}

#[test]
fn single() {
    let board = HexagonalBoard::from([([0, 0], 'a')]);

    print!("{board}");

    let expected = indoc!(
        r"
             /---\
            ⟨  a  ⟩
             \---/
        "
    )
    .trim_end_matches('\n');

    assert_eq!(board.render(), expected);
}

#[test]
fn four() {
    let board =
        HexagonalBoard::from([([0, 0], 'a'), ([1, 0], 'b'), ([0, 1], 'c'), ([-1, -1], 'd')]);

    let expected = indoc!(
        r"
             /---\     /---\
            ⟨  b  ⟩---⟨  c  ⟩
             \---⟨  a  ⟩---/
                  ⟩---⟨
                 ⟨  d  ⟩
                  \---/
        "
    )
    .trim_end_matches('\n');

    assert_eq!(board.render(), expected);
}

#[test]
fn empty_center() {
    let board = HexagonalBoard::from([([1, 1], 't'), ([-1, -1], 'b')]);

    let expected = indoc!(
        r"
             /---\
            ⟨  t  ⟩
             \---/

             /---\
            ⟨  b  ⟩
             \---/
        "
    )
    .trim_end_matches('\n');

    assert_eq!(board.render(), expected);
}
