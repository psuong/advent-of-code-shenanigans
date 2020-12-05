use super::file_util::read_lines;

const TREE : char = '#';
const open_space : char = ' ';

pub fn answer_day_three() {
    if let Ok(lines) = read_lines("resources/day3.txt") {
        for line in lines {
            if let Ok(it) = line {
                for c in it.chars().enumerate() {
                }
                println!("{}", it);
            }
        }
    }
}
