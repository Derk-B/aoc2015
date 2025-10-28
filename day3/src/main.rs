use std::collections::HashMap;
use std::fs;

fn deliver_presents(instructions: impl Iterator<Item = char>) -> HashMap<(i32, i32), i32> {
    let mut houses = HashMap::<(i32, i32), i32>::new();
    let mut position: (i32, i32) = (0, 0);
    *houses.entry(position).or_insert(0) += 1;

    for dir in instructions {
        if dir == '<' {
            position = (position.0 - 1, position.1);
        } else if dir == '>' {
            position = (position.0 + 1, position.1);
        } else if dir == 'v' {
            position = (position.0, position.1 - 1);
        } else if dir == '^' {
            position = (position.0, position.1 + 1);
        } else {
            continue;
        }

        *houses.entry(position).or_insert(0) += 1;
    }

    houses
}

fn main() {
    let file = "input.txt";
    let content = match fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => panic!("Error while trying to read file: {}\n{:?}", file, e),
    };

    println!(
        "Day 3, part 1 = {}",
        deliver_presents(&mut content.chars()).len()
    );

    let santa_directions = &mut content
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, d)| d);
    let robot_directions = &mut content
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, d)| d);
    let mut santa_houses = deliver_presents(santa_directions);
    let robot_houses = deliver_presents(robot_directions);
    for (k, v) in robot_houses.into_iter() {
        *santa_houses.entry(k).or_insert(0) += v;
    }

    println!("Day 3, part 2 = {}", santa_houses.len());
}
