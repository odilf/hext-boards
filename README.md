# hex-boards

A small library that renders hexagonal boards on the terminal. 

## Example

```rust
use hex_boards::HexagonalBoard;
use nalgebra::vector;

let board = HexagonalBoard::from([
	(vector![0, 0], 'a'),
	(vector![1, 0], 'b'),
	(vector![0, 1], 'c'),
	(vector![-1, -1], 'd'),
]);

let output = format!("{board}");

/* Output is the following:

 /---\     /---\
⟨  b  ⟩---⟨  c  ⟩
 \---⟨  a  ⟩---/
      ⟩---⟨
     ⟨  d  ⟩
      \---/
*/
```