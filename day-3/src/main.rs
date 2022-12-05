use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    ops::Deref,
    path::Path,
};

const LOWER_CASE_A_ASCII: u32 = 97;
const LOWER_CASE_Z_ASCII: u32 = 122;
const UPPER_CASE_A_ASCII: u32 = 65;
const UPPER_CASE_Z_ASCII: u32 = 90;

fn main() {
    let input = read_input("input.txt");
    // println!("{}", calculate_sum_of_priorities(Box::new(input))); // part 1
    print!("{}", calculate_sum_of_priorities_per_group(Box::new(input))) // part 2
}

fn read_input(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(Path::new(path)).unwrap();
    return BufReader::new(file).lines().map(Result::unwrap);
}

fn get_item_priority(item: &char) -> u32 {
    let ascii = *item as u32;
    if ascii >= LOWER_CASE_A_ASCII && ascii <= LOWER_CASE_Z_ASCII {
        return ascii - LOWER_CASE_A_ASCII + 1;
    }
    if ascii >= UPPER_CASE_A_ASCII && ascii <= UPPER_CASE_Z_ASCII {
        return ascii - UPPER_CASE_A_ASCII + 27;
    }
    panic!("Nop");
}

fn calculate_sum_of_priorities(rucksacks: Box<dyn Iterator<Item = String>>) -> u32 {
    let mut sum_of_priorities = 0;
    for rucksack in rucksacks {
        let mut items_in_first_compartment: HashSet<&char> = HashSet::new();
        // TODO: Check if this is the best way to solve the problem of
        // referencing stuff of this list from the HashSet, inside the loop.
        let items: Vec<char> = rucksack.chars().collect();
        for i in 0..items.len() {
            let item = items.get(i).unwrap();
            if i < (rucksack.len() / 2) {
                items_in_first_compartment.insert(item);
            } else {
                if items_in_first_compartment.contains(item) {
                    sum_of_priorities += get_item_priority(item);
                    break;
                }
            }
        }
    }
    return sum_of_priorities;
}

fn get_item_available_in_all_items(item_1: &String, item_2: &String, item_3: &String) -> char {
    let set_1: HashSet<char> = HashSet::from_iter(item_1.chars());
    let set_2: HashSet<char> = HashSet::from_iter(item_2.chars());
    let set_3: HashSet<char> = HashSet::from_iter(item_3.chars());
    let set_3_cast: HashSet<&char> = set_3.iter().collect();

    let intersection: HashSet<&char> = set_1.intersection(&set_2).collect();
    let intersection2: HashSet<&&char> = intersection.intersection(&set_3_cast).collect();
    let elements_in_common: Vec<&&&char> = intersection2.iter().collect();

    return elements_in_common
        .first()
        .unwrap()
        .deref()
        .deref()
        .deref()
        .clone();
}

fn calculate_sum_of_priorities_per_group(mut rucksacks: Box<dyn Iterator<Item = String>>) -> u32 {
    let mut total_priorities: u32 = 0;
    while let Some(item_1) = rucksacks.next() {
        let item_2 = rucksacks.next().unwrap();
        let item_3 = rucksacks.next().unwrap();
        let item_in_3_rucksacks = get_item_available_in_all_items(&item_1, &item_2, &item_3);
        total_priorities += get_item_priority(&item_in_3_rucksacks);
    }
    return total_priorities;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        let result = calculate_sum_of_priorities(Box::new(input.split("\n").map(str::to_string)));
        assert!(result == 157, "Expected 157, got {}", result);
    }
    #[test]
    fn part_2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        let result =
            calculate_sum_of_priorities_per_group(Box::new(input.split("\n").map(str::to_string)));
        assert!(result == 70, "Expected 70, got {}", result);
    }
}
