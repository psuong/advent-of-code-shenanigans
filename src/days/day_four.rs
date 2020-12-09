use super::file_util::read_lines;
use regex::Regex;

const FIELD : [&str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
const EYE_COLOR : [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
const REQUIRED_MASK : i32 = 0b0111_1111;
const CID_MASK : i32 = 0b1000_0000;

const DELIMITER : char = ' ';
#[allow(dead_code)]
const PAIR_DELIMITER : char = ':';
#[allow(dead_code)]
const INCH : &str = "in";
#[allow(dead_code)]
const CENTIMETER : &str = "cm";

fn scan_words(words : &Vec<&str>, mask : &mut i32) {
    for word in words {
        for i in 0..FIELD.len() {
            let term = &FIELD[i];
            
            if word.contains(term) {
                let idx = *&i as i32;
                if is_data_valid(&idx, &word) {
                    *mask |= 1 << i;
                    break;
                }
            }
        }
    }
}

fn is_data_valid(index : &i32, word : &str) -> bool {
    let mut split = word.split(PAIR_DELIMITER).enumerate();
    let (_size, value) = split.nth(1).unwrap();
    match index {
        0 => {
            match value.parse::<i32>() {
                Ok(n) => return n >= 1920 && n <= 2002,
                Err(_err) => return false
            }
        },
        1 => {
            match value.parse::<i32>() {
                Ok(n) => return n >= 2010 && n <= 2020,
                Err(_err) => return false
            }
        },
        2 => {
            match value.parse::<i32>() {
                Ok(n) => return n >= 2020 && n <= 2030,
                Err(_err) => return false
            }
        },
        3 => {
            let is_inch = value.contains(INCH);
            let is_cm = value.contains(CENTIMETER);

            let height = value.replace(INCH, "").replace(CENTIMETER, "").parse::<i32>();

            if is_inch {
                match height {
                   Ok(n) => return n >= 59 && n <= 76,
                   Err(_err) => return false
                }
            }

            if is_cm {
                match height {
                    Ok(n) => return n >= 150 && n <= 193,
                    Err(_err) => return false
                }
            }

            false
        },
        4 => {
            let pattern = Regex::new(r"^#[a-f0-9]{6}").unwrap();
            let is_match = pattern.is_match(value);
            return is_match
        },
        5 => {
            for eye_color in EYE_COLOR.iter() {
                if value.contains(eye_color) {
                    return true
                }
            }

            return false
        },
        6 => {
            let pattern = Regex::new(r"^[0-9]{9}").unwrap();
            return pattern.is_match(value)
        },
        7 => {
           true 
        },
        _ => {
            return false
        }
    }
}

pub fn answer_day_four() {
    if let Ok(lines) = read_lines("resources/day4.txt") {
        let mut mask : i32 = 0;
        let mut valid_passports = 0;

        for (_line_number, line) in lines.enumerate() {
            let line_content = line.unwrap();
            let line_length = line_content.len() as i64;
            let words = line_content.split(DELIMITER).collect();

            scan_words(&words, &mut mask);

            if line_length == 0 {
                let min_mask = mask ^ REQUIRED_MASK;
                let condition = min_mask == CID_MASK || min_mask == 0;

                // println!("Mask: {:#010b}, REQUIRED: {:#010b} Cond: {}", mask, REQUIRED_MASK, condition);
                if condition {
                    valid_passports += 1;
                }
                mask = 0;
            };
        }
        
        let min_mask = mask & REQUIRED_MASK;
        let condition = min_mask == CID_MASK || min_mask == 0;

        if condition {
            // println!("Mask: {:#010b}, REQUIRED: {:#010b} Cond: {}", mask, REQUIRED_MASK, condition);
            valid_passports += 1;
        }

        println!("Valid passports: {}", valid_passports);
    }
}
