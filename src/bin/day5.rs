#![allow(dead_code)]

use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
    fs::File,
    io::BufRead,
    time::Instant,
};

fn is_valid(rule_list: &HashMap<u16, HashSet<u16>>, num_list: &Vec<u16>) -> bool {
    let mut seen = HashSet::new();

    for num in num_list.iter() {
        for required_num in rule_list.get(num).unwrap() {
            if seen.contains(required_num) {
                return false;
            }
        }

        seen.insert(*num);
    }

    true
}

fn part_one(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut total = 0;
    let mut rule_map: HashMap<u16, HashSet<u16>> = HashMap::new();

    for line in reader.lines() {
        let line: String = line.unwrap();

        if line.is_empty() {
            continue;
        } else if line.contains("|") {
            let num_list = line
                .split("|")
                .map(|e| e.parse::<u16>().unwrap())
                .collect::<Vec<u16>>();

            if rule_map.contains_key(&num_list[0]) == false {
                rule_map.insert(num_list[0], HashSet::new());
            }

            rule_map.get_mut(&num_list[0]).unwrap().insert(num_list[1]);
        } else {
            let num_list = line
                .split(",")
                .map(|e| e.parse::<u16>().unwrap())
                .collect::<Vec<u16>>();

            if is_valid(&rule_map, &num_list) {
                total += num_list[num_list.len() / 2];
            }
        }
    }

    println!("PART ONE: {}", total);
}

fn fix_num_list(rule_map: &HashMap<u16, HashSet<u16>>, num_list: &mut Vec<u16>) {
    let mut seen = HashMap::new();
    let mut i = 0;

    while i < num_list.len() {
        let num = num_list[i];
        let mut valid = true;

        for required_num in rule_map.get(&num).unwrap() {
            if seen.contains_key(required_num) {
                // swap current number with the required number
                let required_num_index = seen.get(required_num).unwrap();
                num_list.swap(i, *required_num_index);
                valid = false;
                break;
            }
        }

        if valid {
            seen.insert(num, i);
            i += 1;
        } else {
            i = 0;
            seen.clear();
        }
    }
}

fn part_two(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut total = 0;
    let mut rule_map: HashMap<u16, HashSet<u16>> = HashMap::new();

    for line in reader.lines() {
        let line: String = line.unwrap();

        if line.is_empty() {
            continue;
        } else if line.contains("|") {
            let num_list = line
                .split("|")
                .map(|e| e.parse::<u16>().unwrap())
                .collect::<Vec<u16>>();

            if rule_map.contains_key(&num_list[0]) == false {
                rule_map.insert(num_list[0], HashSet::new());
            }

            rule_map.get_mut(&num_list[0]).unwrap().insert(num_list[1]);
        } else {
            let mut num_list = line
                .split(",")
                .map(|e| e.parse::<u16>().unwrap())
                .collect::<Vec<u16>>();

            if is_valid(&rule_map, &num_list) == false {
                fix_num_list(&rule_map, num_list.borrow_mut());
                total += num_list[num_list.len() / 2];
            }
        }
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
