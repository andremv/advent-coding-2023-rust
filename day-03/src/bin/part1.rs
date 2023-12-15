fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn check_char(input: char) -> bool {
    return input.is_digit(10) == false && input != '.';
}

fn part1(input: &str) -> String {
    let mut answer: u32 = 0;

    let mut map: Vec<Vec<char>> = Vec::new();
    let lines: Vec<&str> = input.lines().map(|x| x.trim()).collect();
    for line in lines {
        map.push(line.chars().collect());
    }

    for i in 0..map.len() {
        let mut valid: bool = false;
        let mut buf: u32 = 0;

        for j in 0..map[i].len() {
            if map[i][j].is_digit(10) {
                buf = buf * 10 + map[i][j].to_digit(10).unwrap_or_default();
                if valid {
                    continue;
                }
                for xi in -1..=1 {
                    for xj in -1..=1 {
                        let nx = i as i32 - xi;
                        let nj = j as i32 - xj;
                        if nx > 0
                            && (nx as usize) < map.len()
                            && nj > 0
                            && (nj as usize) < map[nx as usize].len()
                            && check_char(map[nx as usize][nj as usize])
                        {
                            valid = true;
                            break;
                        }
                    }
                    if valid {
                        break;
                    }
                }
            } else {
                if valid {
                    valid = false;
                    answer += buf;
                }
                buf = 0;
            }
        }
        if valid {
            answer += buf;
        }
    }

    return answer.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "467..114..
                    ...*......
                    ..35..633.
                    ......#...
                    617*......
                    .....+.58.
                    ..592.....
                    ......755.
                    ...$.*....
                    .664.598..",
        );
        assert_eq!(result, "4361");
    }
}
