advent_of_code::solution!(2);

fn parse_reports(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe_report(report: &[u32]) -> bool {
    // Count the number of levels
    let n = report.len();

    // Check base case (single level)
    if n == 1 {
        return true;
    }

    // Determine direction
    let increasing = report[0] < report[1];
    let decreasing = report[0] > report[1];

    // Iterate through pairs of adjacent values
    for i in 0..n - 1 {
        // Check if adjacent levels differ by least one and at most three
        let diff = if report[i] < report[i + 1] {
            report[i + 1] - report[i]
        } else {
            report[i] - report[i + 1]
        };
        if !(1..=3).contains(&diff) {
            return false;
        }

        // Check if the levels are consistently increasing
        if increasing && report[i + 1] <= report[i] {
            return false;
        }

        // Check if the levels are consistently decreasing
        if decreasing && report[i + 1] >= report[i] {
            return false;
        }
    }

    true
}

fn is_safe_report_problem_dampener(report: &[u32]) -> bool {
    let report_status = is_safe_report(report);
    if report_status {
        // If the base report is safe, return true
        return true;
    }

    for i in 0..report.len() {
        // If the base report is not safe, try mutations with each level removed
        let mut report_mutation = report.to_owned();
        report_mutation.remove(i);
        if is_safe_report(&report_mutation) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_reports: u32 = 0;

    // Parse each report (line) into levels (values)
    let reports: Vec<Vec<u32>> = parse_reports(input);
    for report in &reports {
        // Check if the report is safe
        if is_safe_report(report) {
            safe_reports += 1;
        }
    }

    // Return the number of safe reports
    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_reports: u32 = 0;

    // Parse each report (line) into levels (values)
    let reports: Vec<Vec<u32>> = parse_reports(input);
    for report in &reports {
        // Check if the report is safe using the problem dampener
        if is_safe_report_problem_dampener(report) {
            safe_reports += 1;
        }
    }

    // Return the number of safe reports
    Some(safe_reports)
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
        assert_eq!(result, Some(4));
    }
}
