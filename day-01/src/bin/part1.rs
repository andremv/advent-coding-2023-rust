fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut sum: i64 = 0;
    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        let mut first: i64 = -1;
        let mut last: i64 = -1;
        for c in line.chars() {
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
        let result = part1("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, "142");
    }
}
