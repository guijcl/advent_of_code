use std::fs;
use std::io::Result;

// ? operator instead of unwrap()

fn main() -> Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    // part 1
    println!("{}", p1(&contents).unwrap());
    println!("{}", ver2_of_p1(&contents).unwrap());

    // part 2
    println!("{}", p2(&contents).unwrap().iter().sum::<i32>());

    Ok(())
}

fn p1(contents: &str) -> Result<i32> {
    let mut max = 0;
    let mut n = 0;
    for e in contents.lines() {
        if e.is_empty() {
            max = max.max(n);
            n = 0;
        } else {
            n += e.parse::<i32>().unwrap_or(0);
        }
    }
    Ok(max)
}

fn ver2_of_p1(contents: &str) -> Result<i32> {
    let max = contents
        .lines()
        .fold((0, 0), |(max, current), line| {
            if line.is_empty() {
                (max.max(current), 0)
            } else {
                let num = line.parse::<i32>().unwrap_or(0);
                (max, current + num)
            }
        })
        .0;
    Ok(max)
}

fn max_arr(mut max: [i32; 3], current: i32) -> [i32; 3] {
    //if self[0].lt(&current) {
    //    self[0] = current
    //} else if self[1].lt(&current) {
    //    self[1] = current
    //} else {
    //    self[2] = current
    //}
    if max[2].lt(&current) {
        max[2] = current;
        max.sort_by(|a, b| b.cmp(a));
    }
    max
}

fn p2(contents: &str) -> Result<[i32; 3]> {
    let max = contents
        .lines()
        .fold(([0, 0, 0], 0), |(max, current), line| {
            if line.is_empty() {
                (max_arr(max, current), 0)
            } else {
                let num = line.parse::<i32>().unwrap_or(0);
                (max, current + num)
            }
        })
        .0;
    Ok(max)
}
