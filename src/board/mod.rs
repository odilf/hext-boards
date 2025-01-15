use std::{collections::HashMap, fmt::Display};

use glam::{ivec2, IVec2};

mod test;

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

/// A board composed of hexagons.
///
/// You can construct a [`HexagonalBoard`] the same ways you would construct a [`HashMap`],
/// where the keys are something that can be converted to a [`glam::IVec2`] (e.g., `[i32; 2]`).
///
/// The positions are interpreted as hexagonal coordinates, where the basis `[1, 0]` is the
/// hexagon to the left and up, and `[0, 1]` is the one to the right and up. 
///
/// You can render a [`HexagonalBoard`] using [`HexagonalBoard::render`] if `T: Into<char> +
/// Clone`. Otherwise, you can use [`HexagonalBoard::render_with`] to specify how to convert the
/// `T` into a [`char`].
///
/// You can also use [`HexagonalBoard::char_map`] if you want to easily get what character should
/// be printed where, but you want to do the rendering yourself.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use hext_boards::HexagonalBoard;
///
/// // Put an `'A'` at `[0, 0]` and a `'B'` at `[1, 1]`.
/// let board = HexagonalBoard::from([
///     ([0, 0], 'A'),
///     ([1, 1], 'B'),
/// ]);
///
/// let output = board.render();
/// let expected = indoc::indoc!(
///     r"
///          /---\
///         ⟨  B  ⟩
///          ⟩---⟨
///         ⟨  A  ⟩
///          \---/
///     "
/// ).trim_end_matches('\n');
///
/// println!("{output}");
///
/// assert_eq!(output, expected)
/// ```
///
/// Using [`HexagonalBoard::render_with`]:
///
/// ```rust
/// use hext_boards::HexagonalBoard;
///
/// let board = HexagonalBoard::from([
///     ([0, 0], 5),
///     ([0, 1], 13),
///     ([1, 1], 25),
/// ]);
/// 
/// // Everything needs to be one char, so we have to use hexadecimal or some other radix to
/// // output higher numbers. 
/// let output = board.render_with(|n| char::from_digit(*n, 36).expect("`n` is less than 36."));
///
/// let expected = indoc::indoc!(
///     r"
///          /---\      
///         ⟨  p  ⟩---\
///          ⟩---⟨  d  ⟩
///         ⟨  5  ⟩---/
///          \---/      
///     "
/// ).trim_end_matches('\n');
///
/// assert_eq!(output, expected)
/// ```
#[derive(Debug, Clone, Default)]
pub struct HexagonalBoard<T> {
    values: HashMap<IVec2, T>,
}

// TODO: I shouldn't have to depend on `Copy` but whatever
impl<T> HexagonalBoard<T> {
    /// Map from coordinates 
    pub fn char_map(&self, into_char: impl Fn(&T) -> char) -> HashMap<IVec2, char> {
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

        let origin_offset = ivec2(5 * min_x + 3, max_y + 1);

        let mut output = HashMap::new();

        for (hexagonal_coords, value) in &self.values {
            // Get cartesian coordinates
            let cartesian_coords = hexagonal_to_cartesian(*hexagonal_coords) + origin_offset;

            // Insert value in center
            output.insert(cartesian_coords, into_char(value));

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

    /// Renders the hexagonal board into a [`String`], converting the `T`s into [`char`]s using
    /// `into_char`.
    ///
    /// If `T` can easily be converted to a [`char`] (i.e., `T: Into<char> + Copy`), you can
    /// use [`Self::render`].
    pub fn render_with(&self, into_char: impl Fn(&T) -> char) -> String {
        render_char_map(self.char_map(into_char))
    }

    /// Renders the hexagonal board into a [`String`].
    ///
    /// To specify how to convert the `T` into a [`char`], see [`Self::render_with`].
    pub fn render(&self) -> String
    where
        for<'a> char: From<T>,
        T: Copy,
    {
        render_char_map(self.char_map(|t| char::from(*t)))
    }
}

fn render_char_map(char_map: HashMap<IVec2, char>) -> String {
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

impl<P: Into<IVec2>, T, V> From<V> for HexagonalBoard<T>
where
    V: IntoIterator<Item = (P, T)>,
{
    fn from(value: V) -> Self {
        value.into_iter().collect()
    }
}

impl<P: Into<IVec2>, T> FromIterator<(P, T)> for HexagonalBoard<T> {
    fn from_iter<I: IntoIterator<Item = (P, T)>>(iter: I) -> Self {
        Self {
            values: iter
                .into_iter()
                .map(|(position, char)| (position.into(), char))
                .collect(),
        }
    }
}

impl<T> Display for HexagonalBoard<T>
where
    for<'a> char: From<T>,
    T: Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
