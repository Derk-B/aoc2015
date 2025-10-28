use std::fs;

fn main() {
    let file = "input.txt";
    let content = match fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => panic!("Error while trying to read {}\n{:?}", file, e),
    };

    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    const FORBIDDEN_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

    let mut n_nice_strings = 0;

    let mut lines = content.lines();
    while let Some(line) = lines.next() {
        let mut vowel_count = 0;
        for c in line.chars() {
            if VOWELS.contains(&c) {
                vowel_count += 1;
            }
        }
        if vowel_count < 3 {
            continue;
        }

        let mut chars = line.chars().peekable();
        let mut chain_count = 0;
        while let Some(c) = chars.next() {
            if let Some(next) = chars.peek() {
                if c == *next {
                    chain_count += 1;
                }
            } else {
                break;
            }
        }

        if chain_count == 0 {
            continue;
        }

        if FORBIDDEN_STRINGS.iter().any(|s| line.contains(s)) {
            continue;
        }

        n_nice_strings += 1;
    }

    println!("Day 5, part 1 = {}", n_nice_strings);

    n_nice_strings = 0;
    let mut lines = content.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        let chars: Vec<char> = line.chars().collect();
        let mut idx = 0;
        let mut pair_found = false;
        while idx < chars.len() - 1 {
            let pair = (chars[idx], chars[idx + 1]);
            let mut idx1 = idx + 2;
            while idx1 < chars.len() - 1 {
                if (chars[idx1], chars[idx1 + 1]) == pair {
                    pair_found = true;
                    break;
                }
                idx1 += 1;
            }
            idx += 1;
        }

        if !pair_found {
            continue;
        }

        let mut twin_count = 0;
        let chars_vec: Vec<char> = line.chars().collect();
        let mut idx = 0;
        while idx < chars_vec.len() - 2 {
            if chars_vec[idx] == chars_vec[idx + 2] {
                twin_count += 1;
                break;
            }
            idx += 1;
        }

        if twin_count == 0 {
            continue;
        }

        n_nice_strings += 1;
    }
    println!("Day 5, part 1 = {}", n_nice_strings);
}
