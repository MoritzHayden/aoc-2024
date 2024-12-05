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

pub fn part_one(input: &str) -> Option<u32> {
    // Parse input into rules and updates
    let mut sum_pages: u32 = 0;
    let rules = parse_rules(input);
    let updates = parse_updates(input);

    // Loop through the updates from right to left and check the rules
    for update in &updates {
        let mut is_valid_update: bool = true;
        for i in (0..update.len()).rev() {
            // If the update is not valid, mark it as such and break
            if rules.contains_key(&update[i]) {
                for j in (0..i).rev() {
                    if rules[&update[i]].contains(&update[j]) {
                        is_valid_update = false;
                        break;
                    }
                }
            }
            if !is_valid_update {
                break;
            }
        }

        // If the update is valid, add the middle page to the sum
        if is_valid_update {
            sum_pages += get_middle(update);
        }
    }

    Some(sum_pages)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
