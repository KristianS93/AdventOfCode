use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("./inputDay3.txt");

    let lines = input.split("\n");
    let mut score: u32 = 0;
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut counter = 0;
    let mut current_high = 0;
    for line in lines {
        if counter % 3 == 0 {
            counter = 0;
            map.clear();
        }
        let half = line.len() / 2;

        let first_half = &line[..half];

        for first in first_half.chars() {
            if !map.contains_key(&first) {
                map.insert(first.clone(), 1);
            }
        }

        let second_half = &line[half..];
        let mut values_in_second: Vec<char> = Vec::new();
        for second in second_half.chars() {
            if !values_in_second.contains(&second) {
                values_in_second.push(second.clone());
                if map.contains_key(&second) {
                    map.insert(second, 1 + map[&second]);
                }
            }
        }

        if counter == 2 {
            // let max = map.iter().max_by(|a, b| a.1.cmp(&b.1));
            // score_point(*max.unwrap().0);
            // score += score_point(max.unwrap().0);
            // println!("{}", max.unwrap().0)
            for (key, val) in map.iter() {
                if val > &1 {
                    let intermed_score = score_point(key);
                    if intermed_score > current_high {
                        current_high = intermed_score;
                    }
                }
            }
            score += current_high;
        }
        counter += 1;
        // println!("{}", line.chars().nth(0).unwrap())
    }

    println!("{}", score);
    // println!("{}", current_high);
}

fn score_point(value: &char) -> u32 {
    if value.is_ascii_lowercase() {
        println!("In lower: {}, {}", value, *value as u32);
        let x = *value as u32;
        return x - 96;
    } else if value.is_ascii_uppercase() {
        println!("In upper: {}, {}", value, *value as u32);
        let x = *value as u32;
        return x - 38;
    }
    0
    // if character.is_lowercase() {
    //     let val = character.to_digit(10);
    //     println!("{:?}", val);
    // } else if character.is_uppercase() {
    //     let val = character.to_digit(10);
    //     println!("{:?}", val);
    //     // val.unwrap() as i32
    // } else {
    //     unreachable!();
    // }
}
