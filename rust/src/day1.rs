mod helper;

use helper::get_input_string;
use std::collections::HashSet;

fn main() {
    let input = input_transform(&get_input_string(1));
    part1(&input);
    part2(&input);
}

fn input_transform(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.parse().expect("it's an int"))
        .collect()
}

fn part1(input: &[i32]) {
    let freq: i32 = input.iter().sum();
    println!("{}", freq);
}

fn part2(input: &[i32]) {
    let mut freq = 0;
    let mut freq_set = HashSet::new();
    freq_set.insert(freq);
    loop {
        for val in input {
            freq += val;
            if freq_set.contains(&freq) {
                println!("{}", freq);
                return;
            } else {
                freq_set.insert(freq);
            }
        }
    }
}
