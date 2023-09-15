//------------------------------------------------------------//
//                    Sudoku Generator                        //
//                                                            //
// Generates Sudoku puzzles in the terminal using a backtrack //
// algorithm. The base struct contains only a single member   //
// which is a value grid of integers.                         //
//                                                            //
//------------------------------------------------------------//       

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

pub struct Sudoku {
    values: Vec<Vec<i8>>
}

impl Sudoku {
    // Construct a 9x9 grid containing only zeros
    fn new() -> Sudoku {
        Sudoku {values: vec![vec![0; 9]; 9]}
    }

    // Returns a sub-grid of 3x3 at the given coordinate
    fn slice(&self, row: usize, column: usize) -> Vec<Vec<i8>> {
        let row_range = (row / 3 as usize * 3)..(row / 3 as usize * 3 + 3);
        let col_range = (column / 3 as usize * 3)..(column / 3 as usize * 3 + 3);
        let rows = self.values[row_range].to_vec();
        rows.iter().map(|x| x[col_range.clone()].to_vec()).collect()
    }

    // Retrieve the next unfilled square within the grid
    fn get_next_empty(&self) -> Option<Vec<usize>> {
        for i in 0..9 {
            for j in 0..9 {
                if self.values[i][j] == 0 {
                    return Some(vec![i, j]);
                }
            }
        }
        None
    }

    // Randomly fill the grid using a backtrack algorithm
    // the algorithm tries to fill the next empty square,
    // if it fails it returns to the last successful filled
    // value and retries it with the next candidate value
    fn rand_fill_grid(&mut self, mut counter: i32) -> bool {
        let mut elements: Vec<i8> = (1..10).collect();
        elements.shuffle(&mut thread_rng());

        for number in elements {
            counter += 1;

            // Add a cap to prevent program running infinitely
            if counter > 2000 {panic!("Failed to fill grid");}

            let next_cell = match self.get_next_empty() {
                Some(v) => v,
                None => {
                    return true;
                }
            };

            // If the value can be placed, do so then attempt the next
            // fill, if that fails, return to this level and reset the
            // current cell
            if self.can_place(next_cell[0], next_cell[1], &number) {
                self.values[next_cell[0]][next_cell[1]] = number;
                if self.rand_fill_grid(counter) {
                    return true;
                }
                self.values[next_cell[0]][next_cell[1]] = 0;
            }
        }
        false
    }

    // Check if the specified value can be placed in the given cell
    fn can_place(&self, row: usize, column: usize, number: &i8) -> bool {
        if self.values[row][column] != 0 {return false;}
        if self.values[row].iter().any(|x| x==number) {return false;}
        if self.values.iter().map(|x| x[column]).any(|x| x == *number) {
            return false;
        }
        let chunk = self.slice(row, column);
        let invalid = chunk.iter().any(|x| x.iter().any(|y| y == number));

        !invalid
    }

    // Mask out the given number of values to convert the filled grid
    // into a puzzle
    fn hide_values(&mut self, n_vals: usize) -> () {
        if n_vals > 81 {
            panic!("Cannot hide more than max number of values");
        }

        let mut values: Vec<usize> = (0..81).collect();
        values.shuffle(&mut thread_rng());

        for i in 0..n_vals {
            let row = values[i] / 9 as usize;
            let col = values[i] - row * 9;
            self.values[row][col] = 0;
        }
    }
}

impl fmt::Display for Sudoku {
    // Define how the puzzle should be displayed within the terminal
    // interpret any zeros as values to hide
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out_str = "".to_string();
        for (i, row) in self.values.iter().enumerate() {
            if i % 3 == 0 && i > 0 {
            
                out_str += "--------- --------- ---------\n";
            }            
            for (j, val) in row.iter().enumerate() {
                if j % 3 == 0 && j > 0 {out_str += "|";}
                if val.clone() == 0 {
                    out_str += "   ";
                } else {
                    out_str += &format!(" {} ", val).to_string();
                }
            }
            out_str += "\n";
        }
        write!(
            f,
            "{}",
            out_str
        )
    }
}

impl Default for Sudoku {
    // Define a default which is a randomly
    // generated puzzle
    fn default() -> Sudoku {
        let mut out = Sudoku::new();
        let counter: i32 = 0;

        out.rand_fill_grid(counter);
        out.hide_values(60);

        out
    }
}

pub fn create_puzzle() -> Sudoku {
    let mut grid = Sudoku::new();
    grid.rand_fill_grid(0);
    grid
}

pub fn solve_puzzle(values: Vec<Vec<i8>>) -> Sudoku {
    let mut puzzle = Sudoku{values: values};
    puzzle.rand_fill_grid(0);
    puzzle 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grid_slice() -> () {
        let vals = vec![
            (1..10).collect(),
            (10..19).collect(),
            (19..28).collect(),
            (28..37).collect(),
            (37..46).collect(),
            (46..55).collect(),
            (55..64).collect(),
            (64..73).collect(),
            (73..82).collect()
        ];
        let grid = Sudoku {values: vals};
        assert_eq!(grid.slice(3, 3), vec![vec![31,32,33], vec![40,41,42], vec![49,50,51]]);
    }

    #[test]
    fn test_can_place() -> () {
        let mut grid = Sudoku::new();
        grid.values[2][2] = 7;
        assert!(grid.can_place(2, 3, &8));
        assert!(!grid.can_place(2, 3, &7));
    }

    #[test]
    fn test_puzzle_gen() -> () {
        create_puzzle();
    }

    #[test]
    fn test_puzzle_solve() -> () {
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

        solve_puzzle(puzzle_vals);
    }
}