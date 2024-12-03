use std::io::Read;

type N = u64;

/*
    Title: Advent Of Code - Day 1, Part 1
    Author: Tristan
    Date: 03/12/2024
    https://adventofcode.com/2024/day/1#part1
*/
pub fn execute_p1() {
    // Original idea
    // Read the input file
    // Split by line
    // Split by middle whitespace (tabs?)
    // Cast both lists as u32
    // Sort both lists
    // Go through each pair and calculate difference
    // Accumulate different as result
    // Report result

    // New idea
    let file_contents = read_input("src/d1/input.txt").expect("D1 input file not found.");
    let lines = split_lines(&file_contents);
    let parsed = parse_line_numbers(lines);
    let sorted = sort_lists(parsed);

    let diff = sum_differences(sorted);

    println!("Day 1, Part 1. Result is {}", diff)
}

fn read_input(path: &str) -> Result<String, std::io::Error> {
    let mut contents = String::new();

    let _f = std::fs::File::open(path)?.read_to_string(&mut contents)?;

    return Ok(contents);
}

fn split_lines(contents: &str) -> Vec<&str> {
    contents.lines().collect()
}

fn parse_line_numbers(lines: Vec<&str>) -> (Vec<N>, Vec<N>) {
    let mut list_a = Vec::with_capacity(lines.len());
    let mut list_b = Vec::with_capacity(lines.len());

    for element in lines {
        let pair = element
            .split_whitespace()
            .collect::<Vec<&str>>();

        let a = pair[0];
        let b = pair[1];

        let number_a = a
            .parse()
            .expect("Lefthand element cannot be cast to integer");

        let number_b = b
            .parse()
            .expect("Righthand element cannot be cast to integer");

        list_a.push(number_a);
        list_b.push(number_b);
    }

    (list_a, list_b)
}

fn sort_lists(lists: (Vec<N>, Vec<N>)) -> (Vec<N>, Vec<N>) {
    let (mut a, mut b) = lists;

    a.sort();
    b.sort();

    (a, b)
}

fn sum_differences((a, b): (Vec<N>, Vec<N>)) -> N {
    a.iter()
        .zip(b.iter())
        .fold(0, |acc, (a, b)| acc + a.abs_diff(*b))
}

/*
    Title: Advent Of Code - Day 1, Part 2
    Author: Tristan
    Date: 03/12/2024
    https://adventofcode.com/2024/day/1#part2
*/
pub fn execute_p2() {
    let file_contents = read_input("src/d1/input.txt").expect("D1 input file not found.");
    let lines = split_lines(&file_contents);
    let (left, right) = parse_line_numbers(lines);

    let score: usize = left
        .iter()
        .map(|f| (*f as usize) * calc_right_list_occurences(*f, &right))
        .sum();

    println!("Day 1, Part 2. Result is {}", score)
}

/// Calculate the similarity score for an element
fn calc_right_list_occurences(left: N, right_list: &Vec<N>) -> usize {
    right_list
        .iter()
        .filter(|f| **f == left)
        .count()
}
