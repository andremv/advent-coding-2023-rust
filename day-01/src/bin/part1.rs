fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut sum: u32 = 0;
    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        for c in line.chars() {
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
        let result = part1("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, "142");
    }
}
