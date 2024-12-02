advent_of_code::solution!(2);

fn is_safe_report(levels: &[u32]) -> bool {
    // Count the number of levels
    let n = levels.len();

    // Check base case for a single level
    if n == 1 {
        return true;
    }

    // Determine direction
    let increasing = levels[0] < levels[1];
    let decreasing = levels[0] > levels[1];

    // Iterate through pairs of adjacent values
    for i in 0..n - 1 {
        // Check if adjacent levels differ by least one and at most three
        let diff = if levels[i] < levels[i + 1] {
            levels[i + 1] - levels[i]
        } else {
            levels[i] - levels[i + 1]
        };
        if diff < 1 || diff > 3 {
            return false;
        }

        // Check if the levels are consistently increasing
        if increasing && levels[i + 1] <= levels[i] {
            return false;
        }

        // Check if the levels are consistently decreasing
        if decreasing && levels[i + 1] >= levels[i] {
            return false;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_reports: u32 = 0;

    // Parse each report (line) into levels (values)
    for line in input.lines() {
        let levels: Vec<u32> = line
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();

        // Check if the report is safe (satisfies the constraints)
        if is_safe_report(&levels) {
            safe_reports += 1;
        }
    }

    // Return the number of safe reports
    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
