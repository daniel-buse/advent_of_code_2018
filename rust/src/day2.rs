extern crate itertools;
mod helper;

use helper::get_input_string;
use itertools::Itertools;

fn main() {
    let input = input_transform(&get_input_string(2));
    part1(&input);
    part2(&input);
}

fn input_transform(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect()
}

fn has_char_occur_exact_n_times(id: &str, n: usize) -> bool {
    id.chars()
        .any(|c| id.chars().filter(|c2| c == *c2).count() == n)
}

fn has_char_occur_twice(id: &str) -> bool {
    has_char_occur_exact_n_times(id, 2)
}

fn has_char_occur_thrice(id: &str) -> bool {
    has_char_occur_exact_n_times(id, 3)
}

fn part1(input: &[String]) {
    let twice_count = input.iter().filter(|id| has_char_occur_twice(id)).count();
    let thrice_count = input.iter().filter(|id| has_char_occur_thrice(id)).count();
    println!("{}", twice_count * thrice_count);
}

fn common_chars(id1: &str, id2: &str) -> String {
    id1.chars()
        .zip(id2.chars())
        .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
        .collect()
}

fn part2(input: &[String]) {
    for ids in input.iter().combinations(2) {
        let (id1, id2) = (ids[0], ids[1]);
        let commond_part = common_chars(id1, id2);
        if commond_part.len() + 1 == id1.len() {
            println!("{}", commond_part);
            return;
        }
    }
}
