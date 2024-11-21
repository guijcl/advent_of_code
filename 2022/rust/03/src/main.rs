use std::collections::HashSet;
use std::fs::read_to_string;
use std::io;
use std::str::FromStr;

type Item = char;

struct RucksackError;

trait Priority {
    fn priority(&self) -> i32;
}

impl Priority for Item {
    fn priority(&self) -> i32 {
        match self {
            ('a'..='z') => *self as i32 - 'a' as i32 + 1,
            ('A'..='Z') => *self as i32 - 'A' as i32 + 27,
            _ => unreachable!("Invalid item character!"),
        }
    }
}

struct Rucksack {
    compartments: (HashSet<Item>, HashSet<Item>),
}

impl FromStr for Rucksack {
    type Err = RucksackError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (first_half, second_half) = line.split_at(line.len() / 2);
        let compartments: (HashSet<Item>, HashSet<Item>) =
            (first_half.chars().collect(), second_half.chars().collect());
        Ok(Rucksack { compartments })
    }
}

impl Rucksack {
    fn find_repeated_item(&self) -> Option<Item> {
        self.compartments
            .0
            .intersection(&self.compartments.1)
            .copied()
            .next()
    }

    fn all_items(&self) -> HashSet<Item> {
        self.compartments
            .0
            .union(&self.compartments.1)
            .copied()
            .collect()
    }
}

struct Group<'a> {
    rucksacks: &'a [Rucksack],
}

impl<'a> Group<'a> {
    fn new(rucksacks: &'a [Rucksack]) -> Option<Self> {
        if rucksacks.len() == 3 {
            Some(Group { rucksacks })
        } else {
            None
        }
    }

    fn find_badge(&self) -> Option<Item> {
        self.rucksacks
            .iter()
            .map(|r| r.all_items())
            .reduce(|acc, items| acc.intersection(&items).copied().collect())
            .and_then(|common| common.into_iter().next())
    }
}

fn main() -> io::Result<()> {
    let contents: String = read_to_string("input.txt")?;

    let rucksacks: Vec<Rucksack> = contents
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let part1: i32 = rucksacks
        .iter()
        .filter_map(|rs| rs.find_repeated_item())
        .map(|item| item.priority())
        .sum();

    let part2: i32 = rucksacks
        .chunks_exact(3)
        .filter_map(Group::new)
        .filter_map(|group| group.find_badge())
        .map(|item| item.priority())
        .sum();

    //let part2: i32 = rucksacks
    //    .chunks_exact(3)
    //    .map(|chunk| Group::new(chunk))      // Vec<Option<Group>>
    //    .filter(|group| group.is_some())     // Vec<Option<Group>> (only Some values)
    //    .map(|group| group.unwrap())         // Vec<Group>
    //    .map(|group| group.find_badge())     // Vec<Option<Item>>
    //    .filter(|badge| badge.is_some())     // Vec<Option<Item>> (only Some values)
    //    .map(|badge| badge.unwrap())         // Vec<Item>
    //    .map(|item| item.priority())         // Vec<u32>
    //    .sum();
    //
    //let part2: i32 = rucksacks
    //    .chunks_exact(3)
    //    .filter_map(|chunk| {
    //        Group::new(chunk)
    //            .and_then(|group| group.find_badge())
    //            .map(|item| item.priority())
    //    })
    //    .sum();

    println!("{part1}");
    println!("{part2}");

    Ok(())
}

//let alphabet_hash: HashMap<char, i32> = ('a'..='z')
//    .chain('A'..='Z')
//    .enumerate()
//    .map(|(i, c)| (c, i as i32 + 1))
//    .collect();

//let points: i32 = contents
//    .lines()
//    .filter_map(Rucksack::new_rucksack)
//    .map(|rs| alphabet_hash[&rs.repeated_item])
//    .sum();
