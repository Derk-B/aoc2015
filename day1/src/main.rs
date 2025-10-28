use std::fs;
fn main() {
    let file = "input.txt";
    let content = match fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => panic!("Error while trying to read {}.\n{:?}", file, e),
    };

    let mut count = 0;
    let mut basement_entered = false;
    let mut idx = 1;
    for c in content.chars() {
        if c == '(' {
            count += 1;
        } else if c == ')' {
            count -= 1;
        }

        if !basement_entered && count < 0 {
            basement_entered = true;
        }

        if !basement_entered {
            idx += 1;
        }
    }
    println!("Day 1, part 1 = {}", count);
    println!("Day 1, part 2 = {}", idx);
}
