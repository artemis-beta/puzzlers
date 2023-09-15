//------------------------------------------------------------//
//                    Wordsearch Generator                    //
//                                                            //
// Generates Wordsearch puzzles in the terminal by checking   //
// for empty cells and determining which directions are valid //
// for placement of specific word.                            //
//                                                            //
//------------------------------------------------------------//

use itertools::Itertools;
use rand::{thread_rng, Rng, seq::SliceRandom, distributions::{Distribution, Standard}};
use std::fmt;

enum Direction {
    Down,
    Up,
    Right,
    Left,
    DiagDown,
    DiagUp
}

pub struct Wordsearch {
    grid: Vec<Vec<char>>
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..6) {
            0 => Direction::Right,
            1 => Direction::Left,
            2 => Direction::Up,
            3 => Direction::Down,
            4 => Direction::DiagUp,
            _ => Direction::DiagDown
        }
    }
}

impl Wordsearch {
    fn new(width: usize, height: usize) -> Wordsearch {
        Wordsearch{grid: vec![vec![' '; width]; height]}
    }
    fn valid_directions(&mut self, word: String, row: usize, column: usize) -> Option<Vec<Direction>> {

        let mut valid_dir = Vec::<Direction>::new();

        let mut pos_x = false;
        let mut neg_x = false;
        let mut pos_y = false;
        let mut neg_y = false;

        if row + word.len() < self.grid.len() {
            let column_vals = &self.grid
               .iter()
               .map(|x| x[column])
               .collect::<Vec<char>>()[row..row + word.len()];
            
            let n_compatible = &word.chars().zip(column_vals.to_vec()).filter(|(a, b)| a == b || b == &' ').count();
            pos_x = n_compatible == &word.len();
        }

        if row as i8 - word.len() as i8 >= 0 {
            let column_vals = &self.grid
                .iter()
                .map(|x| x[column])
                .collect::<Vec<char>>()[row- word.len()..row];

            let n_compatible = &word.chars().zip(column_vals.to_vec()).filter(|(a, b)| a == b || b == &' ').count();
            neg_x = n_compatible == &word.len();
        }

        if column as i8 - word.len() as i8 >= 0 {
            let row_vals = &self.grid[row][column - word.len()..column].to_vec();

            let n_compatible = &word.chars().zip(row_vals.to_vec()).filter(|(a, b)| a == b || b == &' ').count();
            neg_y = n_compatible == &word.len();
        }
        if column + word.len() < self.grid[0].len() {
            let row_vals = &self.grid[row][column..column + word.len()].to_vec();

            let n_compatible = &word.chars().zip(row_vals.to_vec()).filter(|(a, b)| a == b || b == &' ').count();
            pos_y = n_compatible == &word.len();
        }

        if pos_x {
            valid_dir.push(Direction::Right);
            if pos_y {valid_dir.push(Direction::DiagDown);}
        }
        if pos_y {valid_dir.push(Direction::Down);}
        if neg_x {valid_dir.push(Direction::Left);}
        if neg_y {
            valid_dir.push(Direction::Up);
            if pos_x {valid_dir.push(Direction::DiagUp);}
        }
        if valid_dir.len() > 0 {return Some(valid_dir);}
        None
    }
    fn get_candidates(&self, word: &String) -> Option<Vec<Vec<usize>>> {
        let word_chars: Vec<char> = word.chars().collect();
        let coords = (0..self.grid.len()).combinations(2);

        let valid: Vec<Vec<usize>> = coords.filter(|x| self.grid[x[0]][x[1]] == ' ' || self.grid[x[0]][x[1]] == word_chars[0]).collect();

        if valid.len() > 0 {return Some(valid);}
        None
    }
    fn fill_blanks(&mut self) -> () {
        let alpha: Vec<char> = String::from_utf8((b'a'..b'z').collect()).unwrap().chars().collect();
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                if self.grid[i][j] == ' ' {
                    match alpha.choose(&mut thread_rng()) {
                        Some(v) => self.grid[i][j] = *v,
                        None => panic!("Failed to fill gap")
                    };
                }
            }
        }
    }
    fn place_word(&mut self, word: &String, row: usize, column: usize, direction: &Direction) -> () {
        if word.len() < 2 || (word.len() > self.grid[0].len() && word.len() > self.grid.len()) {
            panic!("Word '{}' is invalid length for {}x{} puzzle", word, self.grid[0].len(), self.grid.len());
        }
        for (i, c) in word.chars().enumerate() {
            let coords: Vec<usize> = match direction {
                Direction::Down => {
                    vec![row, column + i]
                },
                Direction::Up => {
                    vec![row, column - i]
                },
                Direction::Right => {
                    vec![row + i, column]
                },
                Direction::Left => {
                    vec![row - i, column]
                },
                Direction::DiagUp => {
                    vec![row + i, column - i]
                },
                Direction::DiagDown => {
                    vec![row + i, column + i]
                }
            };
            self.grid[coords[0]][coords[1]] = c;
        }
    }
}

impl fmt::Display for Wordsearch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out_str = "".to_string();
        for row in self.grid.iter() {
            for val in row {
                out_str += &format!(" {} ", val).to_string();
            }
            out_str += "\n";
        }
        write!(f, "{}", out_str)
    }
}

pub fn create_puzzle(words: &Vec<String>, width: usize, height: usize) -> Wordsearch {
    let mut puzzle = Wordsearch::new(width, height);

    for word in words {
        let mut candidate_coords = match puzzle.get_candidates(&word) {
            Some(v) => v,
            None => panic!("No valid cells found!")
        };
        candidate_coords.shuffle(&mut thread_rng());

        for coord in candidate_coords {
            let mut valid_dir: Vec<Direction> = match puzzle.valid_directions(word.to_string(), coord[0], coord[1]) {
                Some(v) => v,
                None => continue
            };
            valid_dir.shuffle(&mut thread_rng());
            puzzle.place_word(&word.to_string(), coord[0], coord[1], &valid_dir[0]);
            break;
        }
    }

    puzzle.fill_blanks();

    puzzle
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wordsearch_generate() -> () {
        let word_list = vec![
            "airspeed".to_string(),
            "velocity".to_string(),
            "unladen".to_string(),
            "swallow".to_string(),
            "iron".to_string(),
            "oxide".to_string()
        ];
        create_puzzle(&word_list, 20, 20);
    }
}
