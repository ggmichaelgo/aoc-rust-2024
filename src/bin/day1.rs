#![allow(dead_code)]

use std::{collections::HashMap, fs::File, io::BufRead, time::Instant};

fn part_one(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut total = 0;
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in reader.lines() {
        let line: String = line.unwrap();
        let num_list: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|e| e.parse::<i32>().unwrap())
            .collect();

        left_list.push(num_list[0]);
        right_list.push(num_list[1]);
    }

    left_list.sort();
    right_list.sort();

    for (a, b) in left_list.iter().zip(right_list.iter()) {
        total += (a - b).abs();
    }

    println!("PART ONE: {}", total);
}

fn part_two(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut left_list = Vec::new();
    let mut right_list: HashMap<i32, i32> = HashMap::new();

    for line in reader.lines() {
        let line: String = line.unwrap();

        let num_list: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|e| e.parse::<i32>().unwrap())
            .collect();

        left_list.push(num_list[0]);

        let count = right_list.entry(num_list[1]).or_insert(0);
        *count += 1;
    }
    let total = left_list
        .iter()
        .fold(0, |t, x| t + right_list.get(x).unwrap_or(&0) * x);

    println!("PART 2: {}", total);
}

fn main() {
    let start = Instant::now();

    part_one("input");
    part_two("input");

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
