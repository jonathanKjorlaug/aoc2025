use std::fs;

fn get_sign(str: &str) -> i32 {
    let first = str.chars().nth(0).expect("");
    return if first == 'L' { -1 } else { 1 };
}

fn main() {
    let mut states: Vec<i32> = vec![50];

    let contents = fs::read_to_string("input.txt").expect("Test");

    for line in contents.lines() {
        let direction = get_sign(line);
        let times = line[1..].parse().unwrap();

        for i in 0..times {
            states.push((states.last().unwrap() + direction).rem_euclid(100));
        }
    }

    let count = states.iter().filter(|&x| *x == 0).count();

    println!("{count}")
}
