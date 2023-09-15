# Rust Puzzle Generation Library

_PuzzleRS_ is a library for generating puzzles on the command line, currently these include:

* Sudoku
* Wordsearch

## Sudoku

The _Sudoku_ library can be used to both generate and solve puzzles:

### Generate

```rust
use puzzlers::sudoku::create_puzzle;

fn main() -> () {
    println!("{}", create_puzzle());
}
```

### Solve

```rust
use puzzlers::sudoku::solve_puzzle;

fn main() {
  let puzzle_vals = vec![
      vec![6,8,0,0,5,0,0,3,0],
      vec![0,0,5,9,0,0,6,0,4],
      vec![9,4,0,6,3,0,5,2,0],
      vec![0,7,4,3,0,0,2,5,0],
      vec![2,6,9,0,4,5,7,0,3],
      vec![0,5,3,0,0,0,0,4,0],
      vec![0,1,0,0,6,7,0,9,0],
      vec![4,0,6,5,0,3,8,0,0],
      vec![0,0,0,0,1,0,0,6,0]
  ];
  let mut puzzle = solve_puzzle(puzzle_vals);
  println!("{}", puzzle);
}
```

## Wordsearch

Generate a puzzle for a given set of words:

```rust
use puzzlers::wordsearch::create_puzzle;

fn main() -> () {
    let word_list = vec![
        "airspeed".to_string(),
        "velocity".to_string(),
        "unladen".to_string(),
        "swallow".to_string(),
        "iron".to_string(),
        "oxide".to_string()
    ];

    // Create a 20x20 puzzle with these words
    let wordsearch = create_puzzle(word_list, 20, 20);
    println!("{}", wordsearch);
}
```