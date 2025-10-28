use md5;
use std::fs;

fn main() {
    let file = "input.txt";
    let input = match fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => panic!("Error while trying to read {}\n{:?}", file, e),
    };

    let secret_key = input.trim();
    let mut number = 0;
    let mut found_5 = false;
    let mut found_6 = false;
    loop {
        let to_hash = format!("{}{}", secret_key, number);
        let digest = md5::compute(to_hash.clone());

        if format!("{:?}", digest).starts_with("00000") && !found_5 {
            println!("Day 4, part 1 = {}", number);
            found_5 = true;
        }
        if format!("{:?}", digest).starts_with("000000") && !found_6 {
            println!("Day 4, part 2 = {}", number);
            found_6 = true;
        }

        if found_5 && found_6 {
            break;
        }
        number += 1;
    }
}
