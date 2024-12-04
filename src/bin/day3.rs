#![allow(dead_code)]

use std::{fs::File, io::BufRead, time::Instant};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PART_ONE_REGEX: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    static ref PART_TWO_REGEX: Regex =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();
}

fn part_one(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut total = 0;

    for line in reader.lines() {
        let line: String = line.unwrap();

        PART_ONE_REGEX.captures_iter(&line).for_each(|caps| {
            let a = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let b = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            total += a * b;
        });
    }

    println!("PART ONE: {}", total);
}

fn part_two(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut total = 0;
    let mut enabled = true;

    for line in reader.lines() {
        let line: String = line.unwrap();

        PART_TWO_REGEX
            .captures_iter(&line)
            .for_each(|caps| match caps.get(0).unwrap().as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled {
                        let a = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                        let b = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                        total += a * b;
                    }
                }
            });
    }

    println!("PART TWO: {}", total);
}

fn main() {
    let start = Instant::now();

    part_one("input");
    part_two("input");

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
