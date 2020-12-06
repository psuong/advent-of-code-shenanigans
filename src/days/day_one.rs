use std::fs;

fn product_of_two(numbers : &Vec<i32>) -> i32 {
    let mut lhs: i32 = 0;
    let mut rhs: i32 = 0;

    for &n in numbers {
        let diff = 2020 - n;

        for &other in numbers {
            if other == n {
                continue
            }

            if other == diff {
                lhs = n;
                rhs = diff;
            }
        }
    }

    lhs * rhs
}

fn product_of_threes(numbers: &Vec<i32>) -> i32 {
    for &first in numbers {
        for &second in numbers {
            for &third in numbers {
                if first + second + third == 2020 {
                    return first * second * third
                }
            }
        }
    }

    0
}

pub fn answer_day_one() {
    let contents = fs::read_to_string("resources/day1.txt").expect("File can't be opened!");
    let numbers = contents
        .split('\n')
        .into_iter()
        .map(|element| { 
            let n = match element.trim().parse::<i32>() {
                Ok(n) => n,
                Err(_err) => {
                    -1
                }
            };
            n
        })
        .collect::<Vec<i32>>();

    let twos_product = product_of_two(&numbers);
    let threes_product = product_of_threes(&numbers);

    println!("Twos: {}, Threes: {}", twos_product, threes_product);
}
