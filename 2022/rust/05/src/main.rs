use std::{collections::HashMap, fs::read_to_string, io, str::FromStr};

struct CrateError;
struct StackError;
struct MoveError;
struct ProgramError;

#[derive(Clone)]
struct Crate(char);

impl FromStr for Crate {
    type Err = CrateError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.trim().starts_with('[') {
            return Err(CrateError);
        }
        s.chars()
            .nth(1)
            .filter(|&c| c != ' ')
            .map(Crate)
            .ok_or(CrateError)
    }
}

#[derive(Clone)]
struct Stack {
    stacks: HashMap<i32, Vec<Crate>>,
}

impl FromStr for Stack {
    type Err = StackError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stacks = s
            .lines()
            .take_while(|line| line.contains('['))
            .collect::<Vec<_>>()
            .iter()
            .rev()
            .flat_map(|line| {
                line.as_bytes()
                    .chunks(4)
                    .enumerate()
                    .filter_map(|(idx, chunk)| {
                        (chunk[0] == b'[' && chunk[1] != b' ')
                            .then_some((idx as i32 + 1, Crate(chunk[1] as char)))
                    })
            })
            .fold(
                HashMap::<i32, Vec<Crate>>::new(),
                |mut acc, (pos, crate_)| {
                    acc.entry(pos).or_default().push(crate_);
                    acc
                },
            );

        Ok(Stack { stacks })
    }
}

impl Stack {
    fn top_crates(&self) -> Result<String, StackError> {
        let max_key = self.stacks.keys().max().unwrap_or(&0);
        let top = (1..=*max_key)
            .filter_map(|i| {
                self.stacks
                    .get(&i)
                    .and_then(|stack| stack.last().map(|crate_| crate_.0))
                    .filter(|&c| c != ' ')
            })
            .collect();
        Ok(top)
    }

    fn apply_moves(&mut self, moves: &[Move]) {
        for move_ in moves {
            for _ in 0..move_.move_ {
                let crate_to_move = self
                    .stacks
                    .get_mut(&move_.from)
                    .and_then(|stack| stack.pop());

                if let Some(crate_) = crate_to_move {
                    self.stacks.entry(move_.to).or_default().push(crate_);
                }
            }
        }
    }

    fn apply_moves_part2(&mut self, moves: &[Move]) {
        for move_ in moves {
            let from_stack = self.stacks.get_mut(&move_.from);

            let crates_to_move = if let Some(from_vec) = from_stack {
                if from_vec.len() < move_.move_ as usize {
                    continue;
                }
                let split_at = from_vec.len() - move_.move_ as usize;
                from_vec.split_off(split_at)
            } else {
                continue;
            };

            self.stacks
                .entry(move_.to)
                .or_default()
                .extend(crates_to_move);
        }
    }
}

#[derive(Clone)]
struct Move {
    move_: i32,
    from: i32,
    to: i32,
}

#[derive(Clone)]
struct Moves {
    moves: Vec<Move>,
}

impl FromStr for Move {
    type Err = MoveError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let ["move", move_, "from", from, "to", to] = parts.as_slice() else {
            return Err(MoveError);
        };
        Ok(Move {
            move_: move_.parse::<i32>().or(Err(MoveError))?,
            from: from.parse::<i32>().or(Err(MoveError))?,
            to: to.parse::<i32>().or(Err(MoveError))?,
        })
    }
}

impl FromStr for Moves {
    type Err = MoveError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .lines()
            .filter_map(|line| line.parse().ok())
            .collect::<Vec<Move>>();

        Ok(Moves { moves })
    }
}

struct Program {
    stack: Stack,
    moves: Moves,
}

impl FromStr for Program {
    type Err = ProgramError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (stack_lines, moves_lines) = s.split_once("\n\n").ok_or(ProgramError)?;

        Ok(Program {
            stack: stack_lines.parse::<Stack>().or(Err(ProgramError))?,
            moves: moves_lines.parse::<Moves>().or(Err(ProgramError))?,
        })
    }
}

fn main() -> io::Result<()> {
    let contents = read_to_string("input.txt")?;
    let program = contents
        .parse::<Program>()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse the input"))?;

    // Part 1
    let mut stack1 = program.stack.clone();
    stack1.apply_moves(&program.moves.moves);
    let part1 = stack1
        .top_crates()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to get top crates"))?;

    // Part 2
    let mut stack2 = program.stack.clone();
    stack2.apply_moves_part2(&program.moves.moves);
    let part2 = stack2
        .top_crates()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to get top crates"))?;

    println!("{part1}");
    println!("{part2}");

    Ok(())
}
