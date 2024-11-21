// instead of using a static matrix of bools, i could use a hashset
// to keep track of the visited points in the rope struct (TO-DO)

// we could also add method to Directions enum to better follow DRY for pattern matching

use std::{fs::read_to_string, io, str::FromStr};

struct DirectionsError;

type Row = i32;
type Col = i32;

struct Rope {
    knots: Vec<(Row, Col)>,
}

impl Rope {
    fn move_rope(&mut self, matrix: &mut Vec<Vec<bool>>, dir: Directions) {
        let matrix_dims = (matrix[0].len() as i32, matrix.len() as i32);
        self.move_head(matrix_dims, &dir);
        self.knots = std::iter::once(self.knots[0])
            .chain(self.knots.windows(2).enumerate().map(|(i, window)| {
                Rope::move_tail(window[1], window[0], matrix, i == self.knots.len() - 2)
            }))
            .collect::<Vec<(Row, Col)>>();
        match dir {
            Directions::Left(s) => {
                if s > 1 {
                    self.move_rope(matrix, Directions::Left(s - 1));
                }
            }
            Directions::Up(s) => {
                if s > 1 {
                    self.move_rope(matrix, Directions::Up(s - 1));
                }
            }
            Directions::Right(s) => {
                if s > 1 {
                    self.move_rope(matrix, Directions::Right(s - 1));
                }
            }
            Directions::Down(s) => {
                if s > 1 {
                    self.move_rope(matrix, Directions::Down(s - 1));
                }
            }
        }
    }

    fn move_head(&mut self, matrix_dims: (i32, i32), dir: &Directions) {
        let (h_row, h_col) = self.knots[0];
        let (m_row_len, m_col_len) = matrix_dims;
        match dir {
            Directions::Left(_) => {
                if h_row > 0 {
                    self.knots[0] = (h_row - 1, h_col);
                }
            }
            Directions::Up(_) => {
                if h_col > 0 {
                    self.knots[0] = (h_row, h_col - 1)
                }
            }
            Directions::Right(_) => {
                if h_row + 1 < m_row_len {
                    self.knots[0] = (h_row + 1, h_col)
                }
            }
            Directions::Down(_) => {
                if h_col + 1 < m_col_len {
                    self.knots[0] = (h_row, h_col + 1)
                }
            }
        }
    }

    fn move_tail(
        mut curr_knot: (Row, Col),
        previous_knot: (Row, Col),
        matrix: &mut Vec<Vec<bool>>,
        is_last: bool,
    ) -> (Row, Col) {
        let (h_row, h_col) = previous_knot;
        let (t_row, t_col) = curr_knot;
        let row_diff = (h_row - t_row).abs();
        let col_diff = (h_col - t_col).abs();
        if row_diff > 1 || col_diff > 1 {
            curr_knot = (
                t_row + (h_row - t_row).signum(),
                t_col + (h_col - t_col).signum(),
            );
        }
        if is_last {
            let (t_row, t_col) = curr_knot;
            matrix[t_row as usize][t_col as usize] = true;
        }

        curr_knot
    }
}

enum Directions {
    Left(i32),
    Up(i32),
    Right(i32),
    Down(i32),
}

impl FromStr for Directions {
    type Err = DirectionsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut moves = s.split_whitespace();
        if let [Some(dir), Some(steps)] = [moves.next(), moves.next()] {
            match dir {
                "L" => Ok(Directions::Left(
                    steps.parse::<i32>().map_err(|_| DirectionsError)?,
                )),
                "U" => Ok(Directions::Up(
                    steps.parse::<i32>().map_err(|_| DirectionsError)?,
                )),
                "R" => Ok(Directions::Right(
                    steps.parse::<i32>().map_err(|_| DirectionsError)?,
                )),
                "D" => Ok(Directions::Down(
                    steps.parse::<i32>().map_err(|_| DirectionsError)?,
                )),
                _ => Err(DirectionsError),
            }
        } else {
            Err(DirectionsError)
        }
    }
}

fn main() -> io::Result<()> {
    let contents = read_to_string("input.txt")?;
    let start: (Row, Col) = (500, 500);
    let mut matrix_part_1: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000]; //5x6
    let mut matrix_part_2: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000]; //5x6
    matrix_part_1[500][500] = true;
    matrix_part_2[500][500] = true;

    let rope_part_1: Rope = Rope {
        knots: vec![start; 2],
    };

    let rope_part_2: Rope = Rope {
        knots: vec![start; 10],
    };

    contents
        .lines()
        .filter_map(|line| line.parse::<Directions>().ok())
        .fold(rope_part_1, |mut rope, dir| {
            rope.move_rope(&mut matrix_part_1, dir);
            rope
        });

    contents
        .lines()
        .filter_map(|line| line.parse::<Directions>().ok())
        .fold(rope_part_2, |mut rope, dir| {
            rope.move_rope(&mut matrix_part_2, dir);
            rope
        });

    let part_1 = matrix_part_1.iter().flatten().filter(|&&x| x).count();
    let part_2 = matrix_part_2.iter().flatten().filter(|&&x| x).count();

    println!("{part_1}");
    println!("{part_2}");

    Ok(())
}

// old code
//
//match dir {
//    Directions::Left(_) => {
//        if col_diff > 1 {
//            if t_col > h_col {
//                self.tail = (t_row - 1, t_col - 1);
//            } else {
//                self.tail = (t_row - 1, t_col + 1);
//            }
//        }
//    }
//    Directions::Up(_) => {
//        if row_diff > 1 {
//            if t_row > h_row {
//                self.tail = (t_row - 1, t_col - 1);
//            } else {
//                self.tail = (t_row + 1, t_col - 1);
//            }
//        }
//    }
//    Directions::Right(_) => {
//        if col_diff > 1 {
//            if t_col > h_col {
//                self.tail = (t_row + 1, t_col + 1);
//            } else {
//                self.tail = (t_row + 1, t_col - 1);
//            }
//        }
//    }
//    Directions::Down(_) => {
//        if row_diff > 1 {
//            if t_row > h_row {
//                self.tail = (t_row + 1, t_col + 1);
//            } else {
//                self.tail = (t_row - 1, t_col + 1);
//            }
//        }
//    }
//}

// this was uses for part 1, but then part 2 required a more "adaptable" approach
//
//struct Rope {
//    head: (Row, Col),
//    tail: (Row, Col),
//}
//
//impl Rope {
//    fn move_rope(mut self, matrix: &mut Vec<Vec<bool>>, dir: Directions) -> Self {
//        let matrix_dims = (matrix[0].len() as i32, matrix.len() as i32);
//        self = self.move_head(matrix_dims, &dir);
//        self = self.move_tail(matrix);
//        match dir {
//            Directions::Left(s) => {
//                if s > 1 {
//                    self = self.move_rope(matrix, Directions::Left(s - 1));
//                }
//            }
//            Directions::Up(s) => {
//                if s > 1 {
//                    self = self.move_rope(matrix, Directions::Up(s - 1));
//                }
//            }
//            Directions::Right(s) => {
//                if s > 1 {
//                    self = self.move_rope(matrix, Directions::Right(s - 1));
//                }
//            }
//            Directions::Down(s) => {
//                if s > 1 {
//                    self = self.move_rope(matrix, Directions::Down(s - 1));
//                }
//            }
//        }
//        self
//    }
//
//    fn move_head(mut self, matrix_dims: (i32, i32), dir: &Directions) -> Self {
//        let (h_row, h_col) = self.head;
//        let (m_row_len, m_col_len) = matrix_dims;
//        match dir {
//            Directions::Left(_) => {
//                if h_row > 0 {
//                    self.head = (h_row - 1, h_col);
//                }
//            }
//            Directions::Up(_) => {
//                if h_col > 0 {
//                    self.head = (h_row, h_col - 1)
//                }
//            }
//            Directions::Right(_) => {
//                if h_row + 1 < m_row_len - 1 {
//                    self.head = (h_row + 1, h_col)
//                }
//            }
//            Directions::Down(_) => {
//                if h_col + 1 < m_col_len - 1 {
//                    self.head = (h_row, h_col + 1)
//                }
//            }
//        }
//        self
//    }
//
//    fn move_tail(mut self, matrix: &mut Vec<Vec<bool>>) -> Self {
//        let (h_row, h_col) = self.head;
//        let (t_row, t_col) = self.tail;
//        let row_diff = (h_row - t_row).abs();
//        let col_diff = (h_col - t_col).abs();
//        if row_diff > 1 || col_diff > 1 {
//            self.tail = (
//                t_row + (h_row - t_row).signum(),
//                t_col + (h_col - t_col).signum(),
//            );
//        }
//        let (t_row, t_col) = self.tail;
//        matrix[t_row as usize][t_col as usize] = true;
//        self
//    }
//}
