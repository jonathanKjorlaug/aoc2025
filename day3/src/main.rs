use std::fs;
use std::time;

enum InputType {
    Dummy,
    Real,
}

fn first_part(input: InputType) {
    let path = match input {
        InputType::Dummy => "dummy_input.txt",
        InputType::Real => "input.txt",
    };

    let content = fs::read_to_string(path).expect("Test");

    let digits: Vec<Vec<i32>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let mut sum = 0;

    for line in digits.iter() {
        let (_, elements) = line.split_last().unwrap();

        let first_digit = elements.iter().max().unwrap();

        let first_index = elements
            .iter()
            .position(|x| *x == *first_digit)
            .expect("First digit not found")
            + 1;

        let (_, second_elements) = line.split_at(first_index);

        let second_digit = second_elements.iter().max().unwrap();
        sum += first_digit * 10 + second_digit;
    }

    println!("{sum}");
}

fn second_part(input: InputType) {
    let path = match input {
        InputType::Dummy => "dummy_input.txt",
        InputType::Real => "input.txt",
    };

    let content = fs::read_to_string(path).expect("Test");

    let digits: Vec<Vec<i32>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let mut sum = 0;

    for line in digits.iter() {
        let mut start_at = 0;
        for i in 1..13 {
            let (_, mut elements) = line.split_at(start_at);

            elements = elements.split_at(elements.len() - (12 - i)).0;

            let next_digit = elements.iter().max().unwrap();

            start_at += elements
                .iter()
                .position(|x| *x == *next_digit)
                .expect("First digit not found")
                + 1;

            sum += (*next_digit as i64) * 10i64.pow(12 - (i as u32));
        }
    }

    println!("{sum}");
}

fn main() {
    let first_start = time::Instant::now();
    first_part(InputType::Real);

    let elapsed_time = first_start.elapsed();

    let second_start = time::Instant::now();

    second_part(InputType::Real);

    let second_elapsed = second_start.elapsed();

    println!("First part took {} s.", elapsed_time.as_secs_f32());
    println!("Second part took {} s.", second_elapsed.as_secs_f32());
}
