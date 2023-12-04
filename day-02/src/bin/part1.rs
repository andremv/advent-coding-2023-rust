fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut answer: u32 = 0;

    let red_max: u32 = 12;
    let green_max: u32 = 13;
    let blue_max: u32 = 14;

    let mut game_id: u32 = 0;
    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        // not worth parsing the input as it's already sorted
        game_id += 1;
        // did we screw up the input ?
        assert_eq!(line.starts_with("Game"), true);
        // game is playable ?
        let mut game_possible = true;
        // grab the game info
        let game_info: Vec<&str> = line.split(':').collect();
        // bad input ?
        assert_eq!(game_info.len(), 2);
        // split the game sets
        let game_sets: Vec<&str> = game_info[1].split(';').map(|x| x.trim()).collect();
        for game_set in &game_sets {
            let mut red: u32 = 0;
            let mut green: u32 = 0;
            let mut blue: u32 = 0;

            let items: Vec<&str> = game_set.split(',').map(|x| x.trim()).collect();
            for item in items {
                let jk: Vec<&str> = item.split(' ').collect();
                assert_eq!(jk.len(), 2);
                let colour = jk[1];
                let value = jk[0].parse::<u32>().unwrap_or_default();
                match colour {
                    "red" => red = value,
                    "green" => green = value,
                    "blue" => blue = value,
                    &_ => {}
                }
                if red > red_max || green > green_max || blue > blue_max {
                    game_possible = false;
                    break;
                }
            }
            if !game_possible {
                break;
            }
        }
        if game_possible {
            answer += game_id;
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
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "8");
    }
}
