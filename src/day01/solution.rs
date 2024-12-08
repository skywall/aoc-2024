use std::collections::HashMap;
#[path = "../common.rs"]
mod common;

fn unzip_vectors(lines: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let (sources, destinations): (Vec<i32>, Vec<i32>) = lines
        .iter()
        .map(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();

            (
                words[0].parse::<i32>().unwrap(),
                words[1].parse::<i32>().unwrap(),
            )
        })
        .unzip();
    (sources, destinations)
}

#[allow(dead_code)]
pub fn part1() {
    let filename = "src/day01/input.txt";
    let lines = common::load_lines(filename).expect("Unable to load input file.");

    let (mut sources, mut destinations) = unzip_vectors(lines);

    sources.sort();
    destinations.sort();

    let distance = sources
        .iter()
        .zip(destinations.iter())
        .fold(0, |acc, (&a, &b)| acc + (a - b).abs());

    println!("Distance: {}", distance);
}

#[allow(dead_code)]
pub fn part2() {
    let filename = "src/day01/input.txt";
    let lines = common::load_lines(filename).expect("Unable to load input file.");

    let (mut sources, mut destinations) = unzip_vectors(lines);
    let mut destination_counts: HashMap<i32, i32> = HashMap::new();

    destinations.iter().for_each(|&a| {
        *destination_counts.entry(a).or_insert(0) += 1;
    });

    sources.sort();
    destinations.sort();

    let distance = sources
        .iter()
        .map(|&a| destination_counts.get(&a).unwrap_or(&0) * a)
        .sum::<i32>();

    println!("Distance: {}", distance);
}
