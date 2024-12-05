fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut safe: u32 = 0;
    'line: for line in input.lines() {
        let mut direction: Option<i32> = None;
        let levels: Vec<i32> = line.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
        for window in levels.windows(2) {
            let [x, y] = [window[0], window[1]];
            let dir = direction.unwrap_or((y - x).signum());
            if dir == 0 || dir != (y - x).signum() {
                continue 'line;
            }
            if y.abs_diff(x) > 3 || y.abs_diff(x) < 1 {
                continue 'line;
            }
            direction = Some(dir);
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
