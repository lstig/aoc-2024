use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();
        left.push(iter.next().unwrap().parse().unwrap());
        right.push(iter.next().unwrap().parse().unwrap());
    }
    left.sort();
    right.sort();
    for (idx, el) in left.iter().enumerate() {
        sum += el.abs_diff(right[idx]);
    }
    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let mut left: Vec<u32> = Vec::new();
    let mut right: HashMap<u32, u32> = HashMap::new();
    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();
        left.push(iter.next().unwrap().parse().unwrap());
        right.entry(iter.next().unwrap().parse().unwrap()).and_modify(|count| *count += 1).or_insert(1);
    }
    for el in &left {
        sum += el * right.get(el).or(Some(&0)).unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn check_part1() {
        let input = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3"
        };
        let result = part1(input);
        assert_eq!(result, 11)
    }

    #[test]
    fn check_part2() {
        let input = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3"
        };
        let result = part2(input);
        assert_eq!(result, 31)
    }
}
