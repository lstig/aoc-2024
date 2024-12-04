fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut safe: u32 = 0;
    'line: for line in input.lines() {
        let mut last: Option<i32> = None;
        let mut direction: Option<i32> = None;
        for level in line.split_ascii_whitespace() {
            let this : i32 = level.parse().unwrap();
            match last {
                Some(prev) => {
                    let dir = direction.or(Some((this - prev).signum())).unwrap();
                    if dir == 0 || dir != (this - prev).signum() {
                        continue 'line;
                    }
                    if this.abs_diff(prev) > 3 || this.abs_diff(prev) < 1 {
                        continue 'line;
                    }
                    direction = Some((this - prev).signum());
                    last = Some(this);
                }
                _ => last = Some(this),
            }
        }
        safe += 1;
    }
    safe
}

fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn check_part1() {
        let input = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9"
        };
        let result = part1(input);
        assert_eq!(result, 2)
    }

    #[test]
    fn check_part2() {
        let input = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9"
        };
        let result = part2(input);
        assert_eq!(result, 4)
    }
}
