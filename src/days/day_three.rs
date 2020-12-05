use super::file_util::read_lines;
use std::collections::HashSet;

const TREE: char = '#';

fn are_tasks_finished(states : &Vec<bool>)-> bool {
    for state in states {
        if !state {
            return false
        }
    }
    true
}

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

                    coordinate_set.insert(coordinate);
                }
                x += 1
            }

            total_lines += 1;
        }


        let slopes                                    = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut current_coordinates : Vec<(i64, i64)> = vec![(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
        let mut task_states : Vec<bool>               = vec![false, false, false, false, false];
        let mut tree_counts : Vec<i32>                = vec![0, 0, 0, 0, 0];

        while !are_tasks_finished(&task_states) {
            for i in 0..current_coordinates.len() {
                let slope = slopes[i];
                let mut vec2 = current_coordinates[i];

                if vec2.1 < total_lines {
                    vec2.0 = (vec2.0 + slope.0) % line_length;
                    vec2.1 += slope.1;

                    if coordinate_set.contains(&vec2) {
                        tree_counts[i] += 1;
                    }

                    current_coordinates[i] = vec2;
                } else {
                    task_states[i] = true
                }
            }
        }

        let mut product = 1;

        for i in 0..tree_counts.len() {
            println!{"At {}, the # of trees are: {}", i, tree_counts.get(i).unwrap() };
            product *= tree_counts.get(i).unwrap();
        }

        println!("Trees Multiplied: {}", product);
    }
}
