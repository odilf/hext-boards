use super::*;

#[test]
fn display() {
	let board = HexagonalBoard::from([
		(vector![0, 0], 'a'),
		(vector![1, 0], 'b'),
		(vector![0, 1], 'c'),
		(vector![-1, -1], 'd'),
	]);

	let output = format!("{board}");
	let expected = include_str!("./board_example.txt");

	for (output, expected) in output.split('\n').zip(expected.split('\n')) {
		for (output, expected) in expected.chars().zip(output.chars()) {
			assert_eq!(output, expected)
		}
	}
}