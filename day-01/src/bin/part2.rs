use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut sum: i64 = 0;

    let mut num_map: HashMap<String, String> = HashMap::new();
    num_map.insert(String::from("one"), String::from("o1e"));
    num_map.insert(String::from("two"), String::from("t2o"));
    num_map.insert(String::from("three"), String::from("t3e"));
    num_map.insert(String::from("four"), String::from("f4r"));
    num_map.insert(String::from("five"), String::from("f5e"));
    num_map.insert(String::from("six"), String::from("s6x"));
    num_map.insert(String::from("seven"), String::from("s7n"));
    num_map.insert(String::from("eight"), String::from("e8t"));
    num_map.insert(String::from("nine"), String::from("n9e"));

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

        let mut first: i64 = -1;
        let mut last: i64 = -1;
        for c in linex.chars() {
            if let Some(number) = c.to_digit(10) {
                if first == -1 {
                    first = number as i64;
                } else {
                    last = number as i64;
                }
            }
        }
        if first != -1 && last != -1 {
            sum = sum + first * 10 + last;
        } else if first > -1 && last == -1 {
            sum = sum + first * 10 + first;
        }
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
