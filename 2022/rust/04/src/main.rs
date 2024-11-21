use std::{fs::read_to_string, io, str::FromStr};

struct Error;

struct Elf(i32, i32);
struct ElfPair(Elf, Elf);

trait Contains<T> {
    fn contains(&self, other: &T) -> bool;
    fn overlaps(&self, other: &T) -> bool;
}

impl Contains<Elf> for Elf {
    fn contains(&self, other: &Elf) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Elf) -> bool {
        self.0 <= other.1 && self.1 >= other.0
    }
}

impl FromStr for Elf {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let min = parts.next().ok_or(Error)?.parse().or(Err(Error))?;
        let max = parts.next().ok_or(Error)?.parse().or(Err(Error))?;
        Ok(Elf(min, max))
    }
}

impl FromStr for ElfPair {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elfs = s.split(',');
        let elf1 = elfs.next().ok_or(Error)?.parse().or(Err(Error))?;
        let elf2 = elfs.next().ok_or(Error)?.parse().or(Err(Error))?;
        Ok(ElfPair(elf1, elf2))
    }
}

fn main() -> io::Result<()> {
    let contents = read_to_string("input.txt")?;
    let part1: usize = contents
        .lines()
        .filter_map(|line| line.parse().ok())
        .filter(|ElfPair(elf1, elf2)| elf1.contains(elf2) || elf2.contains(elf1))
        .count();

    let part2: usize = contents
        .lines()
        .filter_map(|line| line.parse().ok())
        .filter(|ElfPair(elf1, elf2)| elf1.overlaps(elf2) || elf2.overlaps(elf1))
        .count();

    println!("{part1}");
    println!("{part2}");

    Ok(())
}
