use super::*;

#[test]
fn display() {
    let board = HexagonalBoard::from([
        (ivec2(0, 0), 'a'),
        (ivec2(1, 0), 'b'),
        (ivec2(0, 1), 'c'),
        (ivec2(-1, -1), 'd'),
    ]);

    let output = format!("{board}");
    let expected = include_str!("./board_example.txt");

    println!("{output}");
    println!("{expected}");

    for (output, expected) in output.split('\n').zip(expected.split('\n')) {
        for (output, expected) in expected.chars().zip(output.chars()) {
            assert_eq!(output, expected)
        }
    }
}
