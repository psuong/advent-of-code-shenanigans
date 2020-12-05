use super::file_util::read_lines;
use std::collections::HashSet;

const TREE: char = '#';

pub fn answer_day_three() {
    if let Ok(lines) = read_lines("resources/day3.txt") {
        let mut coordinate_set: HashSet<(i64, i64)> = HashSet::new();

        let mut line_length : i64 = 0;
        let mut total_lines : i64 = 0;

        for (line_number, line) in lines.enumerate() {
            let mut x : i64 = 0;

            let row = &line.unwrap();

            if line_length == 0 {
                line_length = row.len() as i64;
            }

            for (_size, c) in row.chars().enumerate()  {
                if c == TREE {
                    let coordinate = (x, line_number as i64);

                    println!("Added: {}, {}", coordinate.0, coordinate.1);
                    coordinate_set.insert(coordinate);
                }
                x += 1
            }

            total_lines += 1;
        }

        let mut y : i64 = 0;
        let mut x : i64 = 0;

        let mut tree_instances = 0;

        println!("Total lines {}", total_lines);

        while y < total_lines {
            x = (x + 3) % line_length;
            y += 1;

            let coordinate = (x, y);

            if coordinate_set.contains(&coordinate) {
                tree_instances += 1;
            }
        }

        println!("# of trees: {}", tree_instances);
    }
}
