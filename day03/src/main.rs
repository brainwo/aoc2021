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
    let (gamma_rate, epsilon_rate) = (0..file.iter().nth(0).unwrap().len())
        .into_iter()
        .map(|i| {
            file.iter().fold((0u32, 0u32), |(sum_0, sum_1), s| {
                match s.chars().nth(i).unwrap() {
                    '0' => (sum_0 + 1, sum_1),
                    '1' => (sum_0, sum_1 + 1),
                    _ => (sum_0, sum_1),
                }
            })
        })
        .fold(
            (String::new(), String::new()),
            |(gamma, epsilon), (sum_0, sum_1)| match sum_0 > sum_1 {
                true => (format!("{}0", gamma), format!("{}1", epsilon)),
                false => (format!("{}1", gamma), format!("{}0", epsilon)),
            },
        );

    println!(
        "Part one: {}",
        usize::from_str_radix(&gamma_rate, 2).unwrap()
            * usize::from_str_radix(&epsilon_rate, 2).unwrap()
    );
}

fn part_two(file: &Vec<String>) {
    let generator =
        (0..file.iter().nth(0).unwrap().len())
            .into_iter()
            .fold(String::new(), |result, i| {
                match file.iter().filter(|s| s.starts_with(&result)).fold(
                    (0u32, 0u32),
                    |(sum_0, sum_1), s| match s.chars().nth(i).unwrap() {
                        '0' => (sum_0 + 1, sum_1),
                        '1' => (sum_0, sum_1 + 1),
                        _ => (sum_0, sum_1),
                    },
                ) {
                    (sum_0, sum_1) if sum_0 > sum_1 => format!("{}0", result),
                    _ => format!("{}1", result),
                }
            });

    let scrubber =
        (0..file.iter().nth(0).unwrap().len())
            .into_iter()
            .fold(String::new(), |result, i| {
                match file.iter().filter(|s| s.starts_with(&result)).fold(
                    (0u32, 0u32),
                    |(sum_0, sum_1), s| match s.chars().nth(i).unwrap() {
                        '0' => (sum_0 + 1, sum_1),
                        '1' => (sum_0, sum_1 + 1),
                        _ => (sum_0, sum_1),
                    },
                ) {
                    (0, 1) => format!("{}1", result),
                    (1, 0) => format!("{}0", result),
                    (sum_0, sum_1) if sum_0 <= sum_1 => format!("{}0", result),
                    _ => format!("{}1", result),
                }
            });

    println!(
        "Part two: {}",
        usize::from_str_radix(&generator, 2).unwrap()
            * usize::from_str_radix(&scrubber, 2).unwrap()
    );
}
