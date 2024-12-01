use regex::Regex;
use std::cmp::Ordering;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // 1. Parse the input into two lists using regex capturing groups
    let mut total_diff: u32 = 0;
    let mut left_locations: Vec<u32> = Vec::new();
    let mut right_locations: Vec<u32> = Vec::new();
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        left_locations.push(caps[1].parse::<u32>().unwrap());
        right_locations.push(caps[2].parse::<u32>().unwrap());
    }

    // 2. Sort the lists
    left_locations.sort();
    right_locations.sort();

    // 3. Iterate through the lists and calculate the difference between the two corresponding elements
    for (left, right) in left_locations.iter().zip(right_locations.iter()) {
        match left.cmp(right) {
            Ordering::Greater => total_diff += left - right,
            Ordering::Less => total_diff += right - left,
            _ => total_diff += 0,
        }
    }

    // 4. Return the total sum of differences
    Some(total_diff)
}

pub fn part_two(input: &str) -> Option<u32> {
    // 1. Parse the input into two lists using regex capturing groups
    let mut similarity_score: u32 = 0;
    let mut left_locations: Vec<u32> = Vec::new();
    let mut right_locations: Vec<u32> = Vec::new();
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        left_locations.push(caps[1].parse::<u32>().unwrap());
        right_locations.push(caps[2].parse::<u32>().unwrap());
    }

    // 2. Sort the lists
    left_locations.sort();
    right_locations.sort();

    // 3. Iterate through the lists and calculate the similarity scores
    for left in left_locations.iter() {
        similarity_score += left * right_locations.iter().filter(|x| *x == left).count() as u32;
    }

    // 4. Return the total sum of similarity scores
    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
