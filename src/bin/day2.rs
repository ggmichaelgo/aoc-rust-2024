#![allow(dead_code)]

use std::{fs::File, io::BufRead, time::Instant};

fn safe(num_list: &Vec<i32>) -> bool {
    let first_diff = num_list[1] - num_list[0];

    if first_diff == 0 || first_diff.abs() > 3 {
        return false;
    }

    let is_rising = first_diff > 0;

    for i in 1..num_list.len() - 1 {
        let diff = num_list[i + 1] - num_list[i];

        if diff == 0 || diff.abs() > 3 {
            return false;
        }

        if is_rising && diff < 0 {
            return false;
        }

        if !is_rising && diff > 0 {
            return false;
        }
    }

    true
}

fn part_one(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut total = 0;

    for line in reader.lines() {
        let line: String = line.unwrap();
        let num_list: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if safe(&num_list) {
            total += 1;
        }
    }

    println!("PART ONE: {}", total);
}

fn part_two(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut total = 0;

    for line in reader.lines() {
        let line: String = line.unwrap();
        let num_list: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if safe(&num_list) {
            total += 1;
        } else {
            for i in 0..num_list.len() {
                let mut num_list = num_list.clone();
                num_list.remove(i);

                if safe(&num_list) {
                    total += 1;
                    break;
                }
            }
        }
    }

    println!("PART ONE: {}", total);
}

fn main() {
    let start = Instant::now();

    part_one("input");
    part_two("input");

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
