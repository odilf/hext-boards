# hext-boards

A small library to render hexagonal boards on the terminal.

## Example

```rust
use hext_boards::HexagonalBoard;

let board = HexagonalBoard::from([
    ([0, 0], 'a'),
    ([1, 0], 'b'),
    ([0, 1], 'c'),
    ([-1, -1], 'd'),
]);

let output = board.to_string();

// Also works
println!("{board}");

/* Output is the following:

 /---\     /---\
⟨  b  ⟩---⟨  c  ⟩
 \---⟨  a  ⟩---/
      ⟩---⟨
     ⟨  d  ⟩
      \---/
*/
```
