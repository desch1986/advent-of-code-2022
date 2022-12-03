use crate::utils;

pub fn run() {
    let input = utils::read_file_from_current_folder("src/inputs/day03.txt");

    let bags: Vec<&str> = input.lines().collect();

    part1(&bags);
    part2(&bags);
}

fn to_ascii(c: char) -> i32 {
    let ascii = c as i32;
    if ascii >= 65 && ascii <= 90 {
        return ascii - 65 + 27;
    }
    return ascii - 97 + 1;
}

fn part1(bags: &Vec<&str>) {
    let mut priority_sum = 0;

    for rucksack in bags {
        let (compartment_1, compartment_2) = rucksack.split_at(rucksack.len() / 2);
        for c in compartment_2.chars() {
            if compartment_1.contains(c) {
                priority_sum += to_ascii(c);
                break
            }
        }
    }

    println!("(1): {}", priority_sum);
}

fn part2(bags: &Vec<&str>) {
    let mut priority_sum = 0;
    
    for group_index in (0..bags.len()).step_by(3) {
        let bag_1 = bags[group_index];
        let bag_2 = bags[group_index + 1];
        let bag_3 = bags[group_index + 2];
        for c in bag_1.chars() {
            if bag_2.contains(c) && bag_3.contains(c) {
                priority_sum += to_ascii(c);
                break
            }
        }
    }

    println!("(2): {}", priority_sum);
}

