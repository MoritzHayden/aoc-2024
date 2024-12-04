advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn safe_get(puzzle: &[Vec<char>], row_index: Option<usize>, col_index: Option<usize>) -> char {
    if row_index.is_none() || col_index.is_none() {
        return ' ';
    }

    puzzle
        .get(row_index.unwrap())
        .and_then(|row| row.get(col_index.unwrap()))
        .copied()
        .unwrap_or(' ')
}

fn count_occurrences_part_one(puzzle: &[Vec<char>], word: &str) -> u32 {
    let mut occurrences: u32 = 0;
    let word_chars: Vec<char> = word.chars().collect::<Vec<char>>();

    // Collect all possible words starting from each valid location
    for (row_index, row) in puzzle.iter().enumerate() {
        for (col_index, char) in row.iter().enumerate() {
            // Ignore characters that are not the first or last character of the word
            if char != word_chars.first().unwrap() && char != word_chars.last().unwrap() {
                continue;
            }

            let mut mutations: Vec<String> = vec![char.to_string(); 8];
            for i in 1..word.len() {
                //  0. Current location to right
                mutations[0].push(safe_get(puzzle, Some(row_index), col_index.checked_add(i)));

                //  1. Current location to the left
                mutations[1].push(safe_get(puzzle, Some(row_index), col_index.checked_sub(i)));

                //  2. Current location up
                mutations[2].push(safe_get(puzzle, row_index.checked_sub(i), Some(col_index)));

                //  3. Current location down
                mutations[3].push(safe_get(puzzle, row_index.checked_add(i), Some(col_index)));

                //  4. Current location up-right
                mutations[4].push(safe_get(
                    puzzle,
                    row_index.checked_sub(i),
                    col_index.checked_add(i),
                ));

                //  5. Current location up-left
                mutations[5].push(safe_get(
                    puzzle,
                    row_index.checked_sub(i),
                    col_index.checked_sub(i),
                ));

                //  6. Current location down-right
                mutations[6].push(safe_get(
                    puzzle,
                    row_index.checked_add(i),
                    col_index.checked_add(i),
                ));

                //  7. Current location down-left
                mutations[7].push(safe_get(
                    puzzle,
                    row_index.checked_add(i),
                    col_index.checked_sub(i),
                ));
            }

            // Count mutations matching the word
            occurrences += mutations
                .iter()
                .filter(|&mutation| [word].contains(&mutation.as_str()))
                .count() as u32;
        }
    }

    // Return the total number of occurrences
    occurrences
}

fn count_occurrences_part_two(puzzle: &[Vec<char>]) -> u32 {
    let mut occurrences: u32 = 0;

    // Check for two diagonally crossing "MAS" intersecting at "A"
    for (row_index, row) in puzzle.iter().enumerate() {
        for (col_index, char) in row.iter().enumerate() {
            // Ignore characters other than 'A'
            if *char != 'A' {
                continue;
            }

            let chars: Vec<char> = vec![
                //  1. Current location up-right
                safe_get(puzzle, row_index.checked_sub(1), col_index.checked_add(1)),
                //  2. Current location up-left
                safe_get(puzzle, row_index.checked_sub(1), col_index.checked_sub(1)),
                //  3. Current location down-right
                safe_get(puzzle, row_index.checked_add(1), col_index.checked_add(1)),
                //  4. Current location down-left
                safe_get(puzzle, row_index.checked_add(1), col_index.checked_sub(1)),
            ];

            let count_m: usize = chars.iter().filter(|&ch| *ch == 'M').count();
            let count_s: usize = chars.iter().filter(|&ch| *ch == 'S').count();
            if count_m == 2 && count_s == 2 {
                occurrences += 1;
            }
        }
    }

    // Return the total number of occurrences
    occurrences
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle: Vec<Vec<char>> = parse_input(input);
    let occurrences: u32 = count_occurrences_part_one(&puzzle, "XMAS");
    Some(occurrences)
}

pub fn part_two(input: &str) -> Option<u32> {
    let puzzle: Vec<Vec<char>> = parse_input(input);
    let occurrences: u32 = count_occurrences_part_two(&puzzle);
    Some(occurrences)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
