use regex::Regex;

advent_of_code::solution!(3);

enum Part {
    One,
    Two,
}

fn calculate_product(input: &str, part: Part) -> Option<u32> {
    // Define variables
    let mut product: u32;
    let mut result: u32 = 0;

    // Extract each mul instruction using multiple regex capturing groups
    let re = match part {
        Part::One => Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))").unwrap(),
        Part::Two => Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))").unwrap(),
    };

    // Calculate the product of each mul instruction
    for cap in re.captures_iter(input) {
        let left_instruction: u32 = cap[2].parse::<u32>().unwrap();
        let right_instruction: u32 = cap[3].parse::<u32>().unwrap();
        product = left_instruction * right_instruction;
        result += product;
    }

    // Return the total sum of products
    Some(result)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(calculate_product(input, Part::One)?)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(calculate_product(input, Part::Two)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
