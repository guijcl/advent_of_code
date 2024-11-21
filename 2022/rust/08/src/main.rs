use std::{fs::read_to_string, io};

fn main() -> io::Result<()> {
    let contents = read_to_string("input.txt")?;

    let vec = contents.lines().fold(Vec::new(), |mut acc, line| {
        acc.push(line.chars().map(|c| c as i32).collect::<Vec<i32>>());
        acc
    });

    let vec_r_len = vec.len();
    let vec_c_len = vec[0].len();

    let part_1 = vec
        .iter()
        .enumerate()
        .skip(1)
        .take(vec_r_len - 2)
        .flat_map(|(i, row)| {
            let ref_vec = &vec;
            row.iter()
                .enumerate()
                .skip(1)
                .take(vec_c_len - 2)
                .filter_map(move |(j, &val)| {
                    // Check each direction separately
                    let blocked_left = row[..j].iter().any(|&val_r| val <= val_r);

                    let blocked_right = row[j + 1..].iter().any(|&val_r| val <= val_r);

                    let blocked_up = ref_vec[..i]
                        .iter()
                        .map(|c| &c[j])
                        .any(|&val_c| val <= val_c);

                    let blocked_down = ref_vec[i + 1..]
                        .iter()
                        .map(|c| &c[j])
                        .any(|&val_c| val <= val_c);

                    // If NOT blocked from all directions
                    let is_visible = !(blocked_left && blocked_right && blocked_up && blocked_down);

                    if is_visible {
                        Some((i, j, val))
                    } else {
                        None
                    }
                })
        })
        .count()
        + vec_r_len * 2
        + vec_c_len * 2
        - 4;

    let part_2 = vec
        .iter()
        .enumerate()
        .skip(1)
        .take(vec_r_len - 2)
        .flat_map(|(i, row)| {
            let ref_vec = &vec;
            row.iter()
                .enumerate()
                .skip(1)
                .take(vec_c_len - 2)
                .map(move |(j, val)| {
                    // Check each direction separately
                    let left_trees = row[..j]
                        .iter()
                        .rev()
                        .take_while(|val_r| val > val_r)
                        .count();
                    let until_blocked_left = left_trees + if left_trees < j { 1 } else { 0 };

                    let right_trees = row[j + 1..].iter().take_while(|val_r| val > val_r).count();
                    let until_blocked_right = right_trees
                        + if right_trees < row[j + 1..].len() {
                            1
                        } else {
                            0
                        };

                    let up_trees = ref_vec[..i]
                        .iter()
                        .rev()
                        .map(|c| c[j])
                        .take_while(|val_c| val > val_c)
                        .count();
                    let until_blocked_up = up_trees + if up_trees < i { 1 } else { 0 };

                    let down_trees = ref_vec[i + 1..]
                        .iter()
                        .map(|c| c[j])
                        .take_while(|val_c| val > val_c)
                        .count();
                    let until_blocked_down = down_trees
                        + if down_trees < ref_vec[i + 1..].len() {
                            1
                        } else {
                            0
                        };

                    until_blocked_left * until_blocked_right * until_blocked_up * until_blocked_down
                })
        })
        .max()
        .unwrap_or(0);

    println!("{part_1}");
    println!("{part_2}");

    Ok(())
}
