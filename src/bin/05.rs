use std::collections::HashMap;

advent_of_code::solution!(5);

fn parse_rules(input: &str) -> HashMap<u32, Vec<u32>> {
    // Parse the input before the newline (X|Y) into a HashMap of rules
    //  Key: page number (X)
    //  Value: page number rules (Y)
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in input.lines() {
        // Stop when we reach the newline
        if line.is_empty() {
            break;
        }

        // Parse the line into X and Y using regex capture groups
        let re: regex::Regex = regex::Regex::new(r"(\d+)[|](\d+)").unwrap();
        let caps = re.captures(line).unwrap();
        let x = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let y = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();

        // Add the rule to the HashMap
        match rules.get(&x) {
            Some(v) => {
                let mut new_v = v.clone();
                new_v.push(y);
                rules.insert(x, new_v);
            }
            None => {
                rules.insert(x, vec![y]);
            }
        }
    }

    // Return the HashMap of rules
    rules
}

fn parse_updates(input: &str) -> Vec<Vec<u32>> {
    // Parse the input after the newline (A,B,C,...) into a Vec<Vec<u32>> of updates
    let mut updates: Vec<Vec<u32>> = vec![];
    let mut target_reached = false;

    for line in input.lines() {
        // Parse the line into a Vec<u32>
        if target_reached {
            updates.push(line.split(',').map(|x| x.parse::<u32>().unwrap()).collect());
        }

        // Begin after the newline
        if line.is_empty() {
            target_reached = true;
        }
    }

    // Return the Vec<Vec<u32>> of updates
    updates
}

fn get_middle(updates: &[u32]) -> u32 {
    // Return the middle the page of the vec
    let middle = updates.len() / 2;
    updates[middle]
}

fn is_valid_update(rules: &HashMap<u32, Vec<u32>>, update: &[u32]) -> bool {
    // Loop through the update from right to left and check the rules
    for i in (0..update.len()).rev() {
        // If the update is not valid, return false
        if rules.contains_key(&update[i]) {
            for j in (0..i).rev() {
                if rules[&update[i]].contains(&update[j]) {
                    return false;
                }
            }
        }
    }

    // If the update is valid, return true
    true
}

fn get_matching_updates(
    rules: &HashMap<u32, Vec<u32>>,
    updates: &Vec<Vec<u32>>,
    valid: bool,
) -> Vec<Vec<u32>> {
    let mut matching_updates: Vec<Vec<u32>> = vec![];

    // Loop through each update and check if it is valid
    for update in updates {
        // If the update is not valid, return false
        if (is_valid_update(rules, update) && valid) || (!is_valid_update(rules, update) && !valid)
        {
            matching_updates.push(update.to_vec());
        }
    }

    // If the update is valid, return true
    matching_updates
}

fn correct_update(rules: &HashMap<u32, Vec<u32>>, update: &[u32]) -> Vec<u32> {
    let mut corrected_update: Vec<u32> = vec![];

    // Construct the corrected update
    for page in update {
        // TODO: Replace with corrected page
        corrected_update.push(*page);
    }

    // Return the corrected update
    corrected_update
}

pub fn part_one(input: &str) -> Option<u32> {
    // Parse input into rules and updates
    let mut sum_pages: u32 = 0;
    let rules: HashMap<u32, Vec<u32>> = parse_rules(input);
    let updates: Vec<Vec<u32>> = parse_updates(input);

    // Get the valid updates
    let valid_updates: Vec<Vec<u32>> = get_matching_updates(&rules, &updates, true);

    // Sum the middle pages of each valid update
    for valid_update in valid_updates {
        sum_pages += get_middle(&valid_update);
    }

    // Return the sum of valid middle pages
    Some(sum_pages)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Parse input into rules and updates
    let mut sum_pages: u32 = 0;
    let rules: HashMap<u32, Vec<u32>> = parse_rules(input);
    let updates: Vec<Vec<u32>> = parse_updates(input);

    // Get the invalid updates
    let invalid_updates: Vec<Vec<u32>> = get_matching_updates(&rules, &updates, false);

    // Correct the ordering of each invalid update and sum the middle pages
    for invalid_update in invalid_updates {
        let corrected_update: Vec<u32> = correct_update(&rules, &invalid_update);
        sum_pages += get_middle(&corrected_update);
    }

    // Return the sum of corrected middle pages
    Some(sum_pages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
