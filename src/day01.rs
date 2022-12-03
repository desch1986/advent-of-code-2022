use crate::utils;

pub fn run() {
    let input = utils::read_file_from_current_folder("src/inputs/day01.txt");    
    let calories: Vec<Vec<i32>> = get_calorie_list_per_elf(input);
    part1(&calories);
    part2(&calories);
}

fn get_calorie_list_per_elf(input: String) -> Vec<Vec<i32>> {
    let calories_string: Vec<&str> = input.split("\r\n\r\n").collect();
    let calories_by_elf: Vec<Vec<i32>> = calories_string.iter().map(|&val| {
        let calories_string: Vec<&str> = val.lines().collect();
        let calories: Vec<i32> = calories_string.iter().map(|&val| {
            return val.parse::<i32>().unwrap();
        }).collect();
        return calories;
    }).collect();
    return calories_by_elf;
}

fn part1(calories: &Vec<Vec<i32>>) {
    let mut max_calories = 0;
    for e in 0..calories.len() {
        let mut elf_calories = 0;
        for c in 0..calories[e].len() {
            elf_calories += calories[e][c];
        }
        if max_calories < elf_calories {
            max_calories = elf_calories;
        }
    }
    println!("(1): {}", max_calories);
}

fn part2(calories: &Vec<Vec<i32>>) {
    let mut calories_by_elf: Vec<i32> = calories.iter().map(|single_calories| {
        let mut elf_calories = 0;
        for c in 0..single_calories.len() {
            elf_calories += single_calories[c];
        }
        return elf_calories;
    }).collect();

    calories_by_elf.sort();
    calories_by_elf.reverse();
    
    println!("(2): {}", (calories_by_elf[0] + calories_by_elf[1] + calories_by_elf[2]));
}