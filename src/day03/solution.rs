use regex::{Captures, Regex};
#[path = "../common.rs"]
mod common;

#[allow(dead_code)]
pub fn part1() {
    let filename = "src/day03/input.txt";
    let lines = common::load_lines(filename).expect("Unable to load input file.");

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let lines = lines.join("");
    let total = re
        .captures_iter(lines.as_str())
        .map(|x| {
            let first: i64 = x.get(1).unwrap().as_str().parse().unwrap();
            let second: i64 = x.get(2).unwrap().as_str().parse().unwrap();

            first * second
        })
        .sum::<i64>();

    println!("Total: {}", total);
}

#[allow(dead_code)]
pub fn part2() {
    let filename = "src/day03/input.txt";
    let lines = common::load_lines(filename).expect("Unable to load input file.");

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();

    let lines = lines.join("");
    let total =
        re.captures_iter(lines.as_str())
            .fold((true, 0), |mut acc: (bool, i64), x: Captures| {
                let matched = x.get(0).unwrap().as_str();

                if matched == "do()" {
                    acc.0 = true;
                } else if matched == "don't()" {
                    acc.0 = false;
                } else {
                    if acc.0 {
                        let first: i64 = x.get(1).unwrap().as_str().parse().unwrap();
                        let second: i64 = x.get(2).unwrap().as_str().parse().unwrap();

                        acc.1 += first * second;
                    }
                }

                acc
            });

    println!("Total: {}", total.1);
}
