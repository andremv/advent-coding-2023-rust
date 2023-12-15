use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn insert_gear(gears: &mut HashMap<String, Vec<u32>>, key: &String, value: &u32) {
    if gears.contains_key(key) {
        gears.get_mut(key).unwrap().push(*value);
    } else {
        let binding = Vec::from([*value]);
        gears.insert(key.clone(), binding);
    }
}

fn part2(input: &str) -> String {
    let mut gears: HashMap<String, Vec<u32>> = HashMap::new();
    let mut map: Vec<Vec<char>> = Vec::new();
    let lines: Vec<&str> = input.lines().map(|x| x.trim()).collect();
    for line in lines {
        map.push(line.chars().collect());
    }

    for i in 0..map.len() {
        let mut valid: bool = false;
        let mut vkey: String = String::new();
        let mut buf: u32 = 0;

        for j in 0..map[i].len() {
            if map[i][j].is_digit(10) {
                buf = buf * 10 + map[i][j].to_digit(10).unwrap_or_default();
                if valid {
                    continue;
                }
                for xi in -1..=1 {
                    for xj in -1..=1 {
                        let ni = i as i32 - xi;
                        let nj = j as i32 - xj;
                        if ni > 0
                            && (ni as usize) < map.len()
                            && nj > 0
                            && (nj as usize) < map[ni as usize].len()
                            && map[ni as usize][nj as usize] == '*'
                        {
                            vkey = format!("{ni}_{nj}");
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
                    insert_gear(&mut gears, &vkey, &buf);
                }
                buf = 0;
            }
        }
        if valid {
            insert_gear(&mut gears, &vkey, &buf);
        }
    }

    let mut answer: u32 = 0;
    for v in gears.values() {
        if v.len() == 2 {
            answer += v[0] * v[1];
        }
    }

    return answer.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
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
        assert_eq!(result, "467835");
    }
}
