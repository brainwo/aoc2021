use std::fs;

fn main() {
    let file: Vec<String> = fs::read_to_string("input.txt")
        .expect("Error reading")
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect();

    part_one(&file);
    part_two(&file);
}

fn part_one(file: &Vec<String>) {
    let (horizontal, depth) = file
        .iter()
        .fold((0i32, 0i32), |(horizontal, depth), s| match s {
            s if s.starts_with("forward") => (
                horizontal + (s.chars().nth(8).unwrap().to_digit(10).unwrap() as i32),
                depth,
            ),
            s if s.starts_with("down") => (
                horizontal,
                depth + (s.chars().nth(5).unwrap().to_digit(10).unwrap() as i32),
            ),
            s if s.starts_with("up") => (
                horizontal,
                depth - (s.chars().nth(3).unwrap().to_digit(10).unwrap() as i32),
            ),
            _ => (0, 0),
        });
    println!("Part one: {}", horizontal * depth);
}

fn part_two(file: &Vec<String>) {
    let (horizontal, depth, _) =
        file.iter()
            .fold((0i32, 0i32, 0i32), |(horizontal, depth, aim), s| match s {
                s if s.starts_with("forward") => {
                    let n = s.chars().nth(8).unwrap().to_digit(10).unwrap() as i32;
                    (horizontal + n, depth + (n * aim), aim)
                }
                s if s.starts_with("down") => {
                    let n = s.chars().nth(5).unwrap().to_digit(10).unwrap() as i32;
                    (horizontal, depth, aim + n)
                }
                s if s.starts_with("up") => {
                    let n = s.chars().nth(3).unwrap().to_digit(10).unwrap() as i32;
                    (horizontal, depth, aim - n)
                }
                _ => (0, 0, 0),
            });
    println!("Part two: {}", horizontal * depth);
}
