use std::{fs, thread::current};

fn convert_string(str: &str) -> i32 {
    let value: i32 = str[1..].parse().unwrap();
    return value;
}

fn get_sign(str: &str) -> i32 {
    let first = str.chars().nth(0).expect("");
    return if first == 'L' { -1 } else { 1 };
}

fn cumsum(numbers: &Vec<i32>) -> Vec<i32> {
    return numbers
        .iter()
        .scan(0, |acc, x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .collect();
}

fn main() {
    let mut count = 0;

    let test = 1..(-4);

    // println!("{test:?}");

    for i in test {
        println!("{i}");
    }

    // let contents = fs::read_to_string("input.txt").expect("Test");
    let contents = fs::read_to_string("dummy_input.txt").expect("Test");

    let signs: Vec<i32> = contents.lines().map(get_sign).collect();
    let numbers: Vec<i32> = contents.lines().map(convert_string).collect();

    let product: Vec<i32> = signs
        .iter()
        .zip(numbers.iter())
        .map(|(x, y)| x * y)
        .collect();

    let cumulative = cumsum(&product);
    let positions: Vec<i32> = cumulative.iter().map(|x| x + 50).collect();

    for (i, n) in cumulative.iter().enumerate() {
        count += numbers[i] / 100;
        if i == 0 && *n <= 0 {
            count += 1;
        }

        let current_position = (n + 50).rem_euclid(100);
        let next_position = product[i] + current_position;

        println!("Current position {current_position}");
        println!("Next position {next_position}");

        for i in (current_position..next_position) {
            println!("{i}");
        }

        if (current_position..(next_position + 1)).contains(&0) {
            count += 1;
        }
        if (next_position..(current_position + 1)).contains(&0) {
            count += 1;
        }
        // if current_position + product[i] > 99 {
        //     println!("Registered above!");
        //     count += 1;
        // } else if current_position + product[i] < 0 {
        //     println!("Registered below!");
        //     count += 1;
        // }
        // if (n + 50).rem_euclid(100) == 0 {
        //     count += 1;
        // }
    }

    println!("{count}")

    // for t in cumulative {
    // println!("{t + 50}");
    // }

    // for line in contents.lines()
    //{
    //     println!("{}", line.chars().nth(0).expect(""));
    //     println!("{}", convert_string(line));
    // }
}
