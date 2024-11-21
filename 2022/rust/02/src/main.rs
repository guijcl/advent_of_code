use std::{fs, io, str::FromStr};

// A - Rock     (1) - X
// B - Paper    (2) - Y
// C - Scissors (3) - Z

#[derive(Copy, Clone, Debug)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone, Debug)]
enum Outcome {
    Victory = 6,
    Draw = 3,
    Loss = 0,
}

#[derive(Debug)]
enum GameError {
    IoError(io::Error),
    ParseError(String),
}

impl From<io::Error> for GameError {
    fn from(error: io::Error) -> Self {
        GameError::IoError(error)
    }
}

impl FromStr for Move {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(GameError::ParseError(format!("Invalid move: {s}"))),
        }
    }
}

impl FromStr for Outcome {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Victory),
            _ => Err(GameError::ParseError(format!("Invalid outcome: {s}"))),
        }
    }
}

impl Move {
    fn play_against(self, opponent_move: Move) -> Outcome {
        use Move::*;
        use Outcome::*;

        match (opponent_move, self) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Victory,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Loss,
        }
    }

    fn score(self, outcome: Outcome) -> i32 {
        self as i32 + outcome as i32
    }

    fn outcome_to_move(m: Move, o: Outcome) -> Move {
        use Move::*;
        use Outcome::*;

        match (m, o) {
            (Rock, Loss) => Scissors,
            (Rock, Draw) => Rock,
            (Rock, Victory) => Paper,

            (Paper, Loss) => Rock,
            (Paper, Draw) => Paper,
            (Paper, Victory) => Scissors,

            (Scissors, Loss) => Paper,
            (Scissors, Draw) => Scissors,
            (Scissors, Victory) => Rock,
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent: Move,
    player: Move,
}

impl Round {
    fn score(&self) -> i32 {
        self.player.score(self.player.play_against(self.opponent))
    }
}

impl FromStr for Round {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let (Some(m1), Some(m2)) = (parts.next(), parts.next()) else {
            return Err(GameError::ParseError(format!("Invalid round format: {s}")));
        };

        if parts.next().is_some() {
            return Err(GameError::ParseError(format!(
                "Too many parts in round: {s}"
            )));
        }

        Ok(Round {
            opponent: m1.parse()?,
            player: m2.parse()?,
        })
    }
}

struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn from_file(file: &str) -> Result<Self, GameError> {
        let contents = fs::read_to_string(file)?;

        Ok(Game {
            rounds: contents
                .lines()
                .map(str::parse)
                .collect::<Result<Vec<Round>, GameError>>()?,
        })
    }

    fn total_score(&self) -> i32 {
        self.rounds.iter().map(Round::score).sum()
    }

    fn with_strategic_outcomes(file: &str) -> Result<Self, GameError> {
        let contents = fs::read_to_string(file)?;

        let rounds = contents
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let (Some(m), Some(o)) = (parts.next(), parts.next()) else {
                    return Err(GameError::ParseError(format!(
                        "Invalid line format: {line}"
                    )));
                };

                if parts.next().is_some() {
                    return Err(GameError::ParseError(format!(
                        "Too many parts in line: {line}"
                    )));
                }

                let opponent_move: Move = m.parse()?;
                let outcome: Outcome = o.parse()?;
                let player_move: Move = Move::outcome_to_move(opponent_move, outcome);

                Ok(Round {
                    opponent: opponent_move,
                    player: player_move,
                })
            })
            .collect::<Result<Vec<Round>, GameError>>()?;
        Ok(Game { rounds })
    }
}

fn main() -> Result<(), GameError> {
    let part_1: i32 = Game::from_file("input.txt")?.total_score();
    println!("{part_1}");

    let part_2: i32 = Game::with_strategic_outcomes("input.txt")?.total_score();
    println!("{part_2}");

    Ok(())
}

//fn to_move(line: &str) -> Result<(Move, Move), GameError> {
//    let moves: Vec<&str> = line.split_whitespace().collect();
//    if moves.len() != 2 {
//        return Err(GameError::ParseError(format!(
//            "Invalid line format: {line}"
//        )));
//    }
//    Ok((Move::from_str(moves[0])?, Move::from_str(moves[1])?))
//}

//fn to_move_outcome(line: &str) -> Result<(Move, Move), GameError> {
//    let moves: Vec<&str> = line.split_whitespace().collect();
//    if moves.len() != 2 {
//        return Err(GameError::ParseError(format!(
//            "Invalid line format: {line}"
//        )));
//    }
//    let m1: Move = Move::try_from(moves[0])?;
//    Ok((m1, outcome_to_move(m1, get_outcome(moves[1]))))
//}

//impl fmt::Display for StateGame {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match self {
//            StateGame::Victory => write!(f, "Victory"),
//            StateGame::Draw => write!(f, "Draw"),
//            StateGame::Loss => write!(f, "Loss"),
//        }
//    }
//}

//fn rock_paper_scissors(m1: Move, m2: Move) -> i32 {
//    let score = m2 as i32;
//    score
//        + match (m1, m2) {
//            (Move::Rock, Move::Paper)
//            | (Move::Paper, Move::Scissors)
//            | (Move::Scissors, Move::Rock) => StateGame::Victory as i32,
//            (Move::Rock, Move::Rock)
//            | (Move::Paper, Move::Paper)
//            | (Move::Scissors, Move::Scissors) => StateGame::Draw as i32,
//            (Move::Rock, Move::Scissors)
//            | (Move::Paper, Move::Rock)
//            | (Move::Scissors, Move::Paper) => StateGame::Loss as i32,
//        }
//}
