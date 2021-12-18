use itertools::Itertools;
use std::fs;

fn main() {
    let file: Vec<u16> = fs::read_to_string("input.txt")
        .expect("Error reading")
        .trim()
        .lines()
        .map(|s| s.parse::<u16>().expect("Error parsing"))
        .collect();

    part_one(&file);
    part_two(&file);
}

fn part_one(file: &Vec<u16>) {
    println!(
        "Part one: {}",
        file.iter()
            .tuple_windows::<(_, _)>()
            .filter(|(p, n)| n > p)
            .count()
    );
}

fn part_two(file: &Vec<u16>) {
    println!(
        "Part two: {}",
        file.iter()
            .tuple_windows::<(_, _, _)>()
            .map(|(x1, x2, x3)| x1 + x2 + x3)
            .tuple_windows::<(_, _)>()
            .filter(|(p, n)| n > p)
            .count()
    );
}
