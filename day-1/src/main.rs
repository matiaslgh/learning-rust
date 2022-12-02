use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let numbers = get_number_per_line("src/input.txt");
    // print!("{}", get_max_sum_per_group(numbers)); // Solution 1
    print!("{}", get_max_sum_of_top_x(numbers, 3));
}

fn get_number_per_line(file_path: &str) -> Box<dyn Iterator<Item = Option<i32>>> {
    let file = File::open(file_path).unwrap();
    let buffer_reader = BufReader::new(file);
    let numbers = buffer_reader.lines().map(|line_result| {
        let line = line_result.unwrap();
        if line == "".to_string() {
            return None;
        }
        return Some(line.parse::<i32>().unwrap());
    });
    return Box::new(numbers);
}

// Solution for part 1
fn get_max_sum_per_group(numbers: Box<dyn Iterator<Item = Option<i32>>>) -> i32 {
    let mut max_group_value = 0;
    let mut group = 0;
    for number in numbers {
        match number {
            Some(number) => {
                group += number;
            }
            None => {
                if group > max_group_value {
                    max_group_value = group;
                }
                group = 0;
            }
        }
    }
    return max_group_value;
}

// Solution for part 2
fn get_max_sum_of_top_x(
    numbers: Box<dyn Iterator<Item = Option<i32>>>,
    amount_of_groups: i32,
) -> i32 {
    let mut groups: Vec<i32> = Vec::new();
    let mut group = 0;
    for number in numbers {
        match number {
            Some(number) => {
                group += number;
            }
            None => {
                groups.push(group);
                group = 0;
            }
        }
    }

    // The last element of the vector is not added in the previous loop because its last element won't be None
    groups.push(group);

    groups.sort_by(|a, b| b.cmp(a));

    let mut count: i32 = 0;
    let mut total: i32 = 0;
    for value in groups {
        if count < amount_of_groups {
            total += value;
        } else {
            return total;
        }
        count += 1;
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solution_1() {
        let numbers = get_number_per_line("src/test.txt");
        let result = get_max_sum_per_group(numbers);
        assert!(result == 24000);
    }

    #[test]
    fn test_solution_2() {
        let numbers = get_number_per_line("src/test.txt");
        let result = get_max_sum_of_top_x(numbers, 3);
        assert!(result == 45000, "Got {}", result);
    }

    #[test]
    fn test_solution_2_works_like_solution_1_if_second_param_is_one() {
        let numbers = get_number_per_line("src/test.txt");
        let result = get_max_sum_of_top_x(numbers, 1);
        assert!(result == 24000, "Got {}", result);
    }
}
