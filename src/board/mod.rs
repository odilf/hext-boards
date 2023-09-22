use std::{collections::HashMap, fmt::Display};

use glam::{ivec2, IVec2};

const LEFT_BRACKET: char = '⟨';
const RIGHT_BRACKET: char = '⟩';

/// Convert from hexagonal coordinates to cartesian coordinates
fn hexagonal_to_cartesian(hexagonal_coords: IVec2) -> IVec2 {
    let b1 = ivec2(-5, -1);
    let b2 = ivec2(5, -1);

    b1 * hexagonal_coords.x + b2 * hexagonal_coords.y
}

const STATIC_TILE_ELEMENTS: [(IVec2, char); 8] = [
    (ivec2(-1, 1), '-'),
    (ivec2(0, 1), '-'),
    (ivec2(1, 1), '-'),
    (ivec2(-3, 0), LEFT_BRACKET),
    (ivec2(3, 0), RIGHT_BRACKET),
    (ivec2(-1, -1), '-'),
    (ivec2(0, -1), '-'),
    (ivec2(1, -1), '-'),
];

const DYNAMIC_TILE_ELEMENTS: [(IVec2, char, char); 4] = [
    (ivec2(-2, 1), '\\', RIGHT_BRACKET),
    (ivec2(2, 1), '/', LEFT_BRACKET),
    (ivec2(-2, -1), '/', RIGHT_BRACKET),
    (ivec2(2, -1), '\\', LEFT_BRACKET),
];

#[derive(Debug, Clone, Default)]
pub struct HexagonalBoard<T> {
    pub values: HashMap<IVec2, T>,
}

// TODO: I shouldn't have to depend on `Copy` but whatever
impl<T> HexagonalBoard<T>
where
    char: From<T>,
    T: Copy,
{
    /// Creates a map from a position in the terminal to the character it should output
    pub fn char_map(&self) -> HashMap<IVec2, char> {
        let min_x = self
            .values
            .keys()
            .map(|pos| pos.x - pos.y)
            .max()
            .unwrap_or(0);
        let max_y = self
            .values
            .keys()
            .map(|pos| pos.x + pos.y)
            .max()
            .unwrap_or(0);

        let origin_offset = ivec2(5 * min_x + 3, 2 * max_y);

        let mut output: HashMap<IVec2, char> = HashMap::new();

        for (hexagonal_coords, value) in &self.values {
            // Get cartesian coordinates
            let cartesian_coords = hexagonal_to_cartesian(*hexagonal_coords) + origin_offset;

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

pub fn render(char_map: HashMap<IVec2, char>) -> String {
    // Get bounds
    let max_x = char_map.keys().map(|pos| pos.x).max().unwrap_or(0);
    let max_y = char_map.keys().map(|pos| pos.y).max().unwrap_or(0);

    let mut output = String::with_capacity((max_x * max_y) as usize);

    for y in 0..=max_y {
        for x in 0..=max_x {
            output.push(*char_map.get(&ivec2(x, y)).unwrap_or(&' '));
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

impl<const N: usize, T> From<[(IVec2, T); N]> for HexagonalBoard<T> {
    fn from(value: [(IVec2, T); N]) -> Self {
        Self {
            values: HashMap::from(value),
        }
    }
}

impl<T> FromIterator<(IVec2, T)> for HexagonalBoard<T> {
    fn from_iter<I: IntoIterator<Item = (IVec2, T)>>(iter: I) -> Self {
        Self {
            values: iter.into_iter().collect(),
        }
    }
}

impl Display for HexagonalBoard<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

#[cfg(test)]
mod test;
