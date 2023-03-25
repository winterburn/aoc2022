#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    let mut elf_calories: Vec<i32> = vec![];
    let mut total_calories: i32 = 0;
    for calory in input.lines() {
        let calory: i32 = match calory.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                elf_calories.push(total_calories);
                total_calories = 0;
                continue;
            }
        };
        total_calories = total_calories + calory;
    }

    elf_calories
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    let max_value = input.iter().max();

    match max_value {
        Some(max) => return *max,
        None => return 0,
    }
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
    let mut sorted = input.clone();

    sorted.sort_by(|a, b| b.cmp(a));

    sorted[0] + sorted[1] + sorted[2]
}
