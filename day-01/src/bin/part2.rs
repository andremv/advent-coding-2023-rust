use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut sum: u32 = 0;
    let num_map = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e")
    ]);

    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        let mut linex = String::from(line);
        loop {
            let mut last_found_pos: Option<usize> = None;
            let mut last_found_k = "";
            let mut last_found_v = "";
            for (k, v) in num_map.iter() {
                let found_pos = linex.find(k);
                if found_pos.is_some() && found_pos.unwrap() < last_found_pos.unwrap_or(linex.len()) {
                    last_found_pos = found_pos;
                    last_found_k = k;
                    last_found_v = v;
                }
            }
            if last_found_pos.is_none() {
                break;
            } else {
                linex = linex.replacen(last_found_k, last_found_v, 1);
            }
        }

        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        for c in linex.chars() {
            let number = c.to_digit(10);
            if number.is_some() {
                if first.is_none() {
                    first = number;
                }
                last = number;
            }
        }
        sum = sum + first.unwrap_or_default() * 10 + last.unwrap_or_default();
    }

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
        assert_eq!(result, "281");
    }
}
