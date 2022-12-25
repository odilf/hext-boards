use std::{collections::HashMap, fmt::Display};
use nalgebra::{Vector2, vector, Matrix2};

const LEFT_BRACKET: char = '⟨';
const RIGHT_BRACKET: char = '⟩';
                    

/// Convert from hexagonal coordinates to cartesian coordinates
fn hexagonal_to_cartesian(hexagonal_coords: Vector2<i32>) -> Vector2<i32> {
	Matrix2::new(-5, 5, -1, -1) * hexagonal_coords
}

const STATIC_TILE_ELEMENTS: [(Vector2<i32>, char); 8] = [
	(vector![-1,  1], '-'),
	(vector![ 0,  1], '-'),
	(vector![ 1,  1], '-'),

	(vector![-3,  0], LEFT_BRACKET),
	(vector![ 3,  0], RIGHT_BRACKET),

	(vector![-1, -1], '-'),
	(vector![ 0, -1], '-'),
	(vector![ 1, -1], '-'),
];

const DYNAMIC_TILE_ELEMENTS: [(Vector2<i32>, char, char); 4] = [
	(vector![-2,  1], '\\', RIGHT_BRACKET),
	(vector![ 2,  1], '/', LEFT_BRACKET),
	(vector![-2, -1], '/', RIGHT_BRACKET),
	(vector![ 2, -1], '\\', LEFT_BRACKET),
];

#[derive(Debug, Clone, Default)]
pub struct HexagonalBoard<T> {
	pub values: HashMap<Vector2<i32>, T>
}

// TODO: I shouldn't have to depend on `Copy` but whatever
impl<T> HexagonalBoard<T> where char: From<T>, T: Copy {

	/// Creates a map from a position in the terminal to the character it should output
	pub fn char_map(&self) -> HashMap<Vector2<i32>, char> {
		let min_x = self.values.iter().map(|(pos, _)| pos.x - pos.y).max().unwrap_or(0);
		let max_y = self.values.iter().map(|(pos, _)| pos.x + pos.y).max().unwrap_or(0);

		let origin_offset = vector![5 * min_x + 3, 2 * max_y];

		let mut output: HashMap<Vector2<i32>, char> = HashMap::new();

		for (hexagonal_coords, value) in &self.values {
			// Get cartesian coordinates
			let cartesian_coords: Vector2<i32> = hexagonal_to_cartesian(*hexagonal_coords) + origin_offset;

			// Insert value in center
			output.insert(cartesian_coords, char::from(*value));

			// Add top and sides, which are always the same 
			for (offset, char) in STATIC_TILE_ELEMENTS {
				output.insert(cartesian_coords + offset, char);
			}

			// Add edges, which depend if a tile has neighbours
			for (offset, single, multiple) in DYNAMIC_TILE_ELEMENTS {
				match output.get(&(cartesian_coords + offset)) {
					None => output.insert(cartesian_coords + offset, single),
					Some(&RIGHT_BRACKET | &LEFT_BRACKET) => None,
					Some(_) => output.insert(cartesian_coords + offset, multiple),
				};
			}
		}

		output
	}
}

pub fn render(char_map: HashMap<Vector2<i32>, char>) -> String {
	// Get bounds
	let max_x = char_map.iter().map(|(pos, _)| pos.x).max().unwrap_or(0);
	let max_y = char_map.iter().map(|(pos, _)| pos.y).max().unwrap_or(0);

	let mut output = String::with_capacity((max_x * max_y) as usize);

	for y in 0..=max_y {
		for x in 0..=max_x {
			output.push(*char_map.get(&vector![x, y]).unwrap_or(&' '));
		}

		if y != max_y {
			output.push('\n');
		}
	}

	output
}


impl From<HexagonalBoard<char>> for String {
    fn from(board: HexagonalBoard<char>) -> Self {
		render(board.char_map())      
    }
}

impl<const N: usize, T> From<[(Vector2<i32>, T); N]> for HexagonalBoard<T> {
    fn from(value: [(Vector2<i32>, T); N]) -> Self {
        Self { values: HashMap::from(value) }
    }
}

impl Display for HexagonalBoard<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",  String::from(self.clone()))
    }
}

#[cfg(test)]
mod test;