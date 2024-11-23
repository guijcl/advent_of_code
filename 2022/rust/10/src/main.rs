use std::{fmt::Display, fs::read_to_string, io, str::FromStr};

const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;
const SIGNAL_CYCLE_OFFSET: i32 = 20;
const MAX_SIGNAL_CYCLE: i32 = 220;

struct OperationsError;

#[derive(Clone)]
struct Row {
    pixels: Vec<char>,
}

impl Row {
    fn new() -> Self {
        Row {
            pixels: vec!['.'; SCREEN_WIDTH],
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pixels.iter().collect::<String>())
    }
}

struct CRT {
    rows: Vec<Row>,
}

impl CRT {
    fn new() -> Self {
        CRT {
            rows: Vec::with_capacity(SCREEN_HEIGHT),
        }
    }

    fn draw_pixel(&mut self, cycle: i32, sprite_pos: i32) {
        let pos: usize = (cycle % SCREEN_WIDTH as i32) as usize;
        let row: usize = (cycle / SCREEN_WIDTH as i32) as usize;
        if pos == 0 {
            self.rows.push(Row::new())
        }
        if (pos as i32).abs_diff(sprite_pos) <= 1 {
            self.rows[row].pixels[pos] = '#';
        }
    }
}

struct State {
    cycle: i32,
    signal_strengths: Vec<(i32, i32)>,
    register: i32,
    crt: CRT,
}

impl State {
    fn new() -> Self {
        State {
            cycle: 0,
            signal_strengths: Vec::new(),
            register: 1,
            crt: CRT::new(),
        }
    }
    fn process_operation(&mut self, op: &Operations) {
        match op {
            Operations::Addx(v) => {
                self.tick();
                self.tick();
                self.register += v
            }
            Operations::Noop => self.tick(),
        }
    }

    fn tick(&mut self) {
        self.crt.draw_pixel(self.cycle, self.register);
        self.cycle += 1;

        if (self.cycle + SIGNAL_CYCLE_OFFSET) % (SCREEN_WIDTH as i32) == 0 {
            self.signal_strengths
                .push((self.cycle, self.cycle * self.register));
        }
    }
}

enum Operations {
    Addx(i32),
    Noop,
}

impl FromStr for Operations {
    type Err = OperationsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        match (parts.next(), parts.next()) {
            (Some("addx"), Some(value)) => value
                .parse()
                .map(Operations::Addx)
                .map_err(|_| OperationsError),
            (Some("noop"), None) => Ok(Operations::Noop),
            _ => Err(OperationsError),
        }
    }
}

fn main() -> io::Result<()> {
    let mut state: State = State::new();

    read_to_string("input.txt")?
        .lines()
        .filter_map(|line| line.parse().ok())
        .for_each(|op| state.process_operation(&op));

    let part_1: i32 = state
        .signal_strengths
        .iter()
        .take_while(|(c, _)| c <= &MAX_SIGNAL_CYCLE)
        .map(|(_, s)| s)
        .sum();

    println!("{part_1}");
    state
        .crt
        .rows
        .iter()
        .take(SCREEN_HEIGHT)
        .for_each(|row| println!("{row}")); // part_2

    Ok(())
}
