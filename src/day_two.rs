use std::collections::HashMap;
use std::fs;

fn count_ranges(lines: &Vec<&str>) {
    let mut instances = 0;

    for i in 0..lines.len() {
        let line = lines[i];
        let split_line = line.split(' ').into_iter().collect::<Vec<&str>>();

        if split_line.len() < 3 {
            continue;
        }

        let range = split_line[0]
            .split('-')
            .into_iter()
            .map(|element| {
                let n = match element.trim().parse::<i32>() {
                    Ok(n) => n,
                    Err(_err) => -1,
                };
                n
            })
            .collect::<Vec<i32>>();

        let letter = split_line[1].chars().nth(0).unwrap();

        let mut map: HashMap<char, i32> = HashMap::new();
        let size = split_line[2].len();

        for j in 0..size {
            let c = split_line[2].chars().nth(j).unwrap();

            if c == letter {
                if !map.contains_key(&c) {
                    map.insert(c, 1);
                } else {
                    *map.get_mut(&c).unwrap() += 1;
                }
            }
        }

        for key_value in &map {
            let count = key_value.1;

            if count >= &range[0] && count <= &range[1] {
                instances += 1;
            }
        }
    }

    println!("Number of correct ranged instances: {}", instances);
}

pub fn count_index(lines: &Vec<&str>) {

    let mut instances = 0;
    for i in 0..lines.len() {
        let line = lines[i];
        let split_line = line.split(' ').into_iter().collect::<Vec<&str>>();

        if split_line.len() < 3 {
            continue;
        }

        let range = split_line[0]
            .split('-')
            .into_iter()
            .map(|element| {
                let n = match element.trim().parse::<i32>() {
                    Ok(n) => n - 1,
                    Err(_err) => -1,
                };
                n
            })
            .collect::<Vec<i32>>();
        let letter = split_line[1].trim().chars().nth(0).unwrap();

        let first_index = range[0] as usize;
        let second_index = range[1] as usize;

        if split_line[2].len() < second_index {
            continue;
        }

        let chars = split_line[2].trim().chars().collect::<Vec<char>>();

        let first = chars[first_index];
        let second = chars[second_index];
        
        if first == letter && second != letter || second == letter && first != letter
        {
            instances += 1;
        }
    }
    println!("{}", instances);
}

pub fn answer_day_two() {
    let content = fs::read_to_string("resources/day2.txt").expect("File can't open!");
    let lines = content.split('\n').into_iter().collect::<Vec<&str>>();

    count_ranges(&lines);
    count_index(&lines);
}
