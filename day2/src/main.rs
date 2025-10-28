use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

fn parse_dim<'a, T>(line_segments: &mut impl Iterator<Item = &'a str>) -> T
where
    T: FromStr,
    T: FromStr<Err: Debug>,
{
    if let Some(seg) = line_segments.next() {
        match seg.parse::<T>() {
            Ok(n) => return n,
            Err(e) => panic!("{:?}", e),
        }
    } else {
        panic!("Failed to parse dimension from line.");
    }
}

fn main() {
    let path = "input.txt";
    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => panic!("Error while trying to read {}\n{:?}", path, e),
    };

    let mut paper_area = 0;
    let mut total_ribbon_length = 0;
    let mut lines = content.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split('x');
        let w = parse_dim::<i32>(&mut parts);
        let h = parse_dim::<i32>(&mut parts);
        let l = parse_dim::<i32>(&mut parts);

        let sizes = vec![w * h, w * l, l * h];
        let smallest_size = sizes.iter().min().unwrap();

        for size in &sizes {
            paper_area += size * 2;
        }
        paper_area += smallest_size;

        let perimeters = vec![2 * (l + h), 2 * (l + w), 2 * (h + w)];
        let ribbon_length = perimeters.iter().min().unwrap();
        total_ribbon_length += ribbon_length;
        let bow_size = w * h * l;
        total_ribbon_length += bow_size;
    }

    println!("Day 2, part 1 = {}", paper_area);
    println!("Day 2, part 2 = {}", total_ribbon_length);
}
