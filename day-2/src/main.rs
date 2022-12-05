use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = read_input("src/input.txt");
    // print!("{}", calculate_score(Box::new(input))); // part 1
    print!("{}", calculate_score_part_2(Box::new(input))); // part 2
}

fn read_input(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).unwrap();
    let buffer_reader = BufReader::new(file);
    return buffer_reader.lines().map(|line| line.unwrap());
}

fn calculate_score(input: Box<dyn Iterator<Item = String>>) -> i32 {
    let score_map: HashMap<&str, i32> = HashMap::from([
        ("A X", 1 + 3), // Rock - Rock (1) + (3) Draw
        ("B Y", 2 + 3), // Paper - Paper (2) + (3) Draw
        ("C Z", 3 + 3), // Scissors - Scissors (3) + (3) Draw
        ("C X", 1 + 6), // Scissors - Rock (1) + (6) Win
        ("A Y", 2 + 6), // Rock - Paper (1) + (6) Win
        ("B Z", 3 + 6), // Paper - Scissors (1) + (6) Win
        ("B X", 1 + 0), // Paper - Rock (1) + (0) Lose
        ("C Y", 2 + 0), // Scissors - Paper (1) + (0) Lose
        ("A Z", 3 + 0), // Rock - Scissors (1) + (0) Lose
    ]);
    return input.map(|line| score_map.get(line.trim()).unwrap()).sum();
}

fn calculate_score_part_2(input: Box<dyn Iterator<Item = String>>) -> i32 {
    //  X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
    let score_map: HashMap<&str, i32> = HashMap::from([
        ("A X", 3 + 0), // Rock - Scissors
        ("B Y", 2 + 3), // Paper - Paper
        ("C Z", 1 + 6), // Scissors - Rock
        ("C X", 2 + 0), // Scissors - Paper
        ("A Y", 1 + 3), // Rock - Rock
        ("B Z", 3 + 6), // Paper - Scissors
        ("B X", 1 + 0), // Paper - Rock
        ("C Y", 3 + 3), // Scissors - Scissors
        ("A Z", 2 + 6), // Rock - Paper
    ]);
    return input.map(|line| score_map.get(line.trim()).unwrap()).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let input_content = "A Y\nB X\nC Z";
        let input = input_content.split("\n").map(|a| a.to_string());
        let result = calculate_score(Box::new(input));
        assert!(result == 15);
    }

    #[test]
    fn it_works_2() {
        let input_content = "A Y\nB X\nC Z";
        let input = input_content.split("\n").map(|a| a.to_string());
        let result = calculate_score_part_2(Box::new(input));
        assert!(result == 12);
    }
}
