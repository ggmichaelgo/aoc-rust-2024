#![allow(dead_code)]

use lazy_static::lazy_static;
use std::{
    fs::File,
    io::BufRead,
    time::{Duration, Instant},
};

lazy_static! {
    static ref DIRECTION_LIST: Vec<(i32, i32)> = vec![
        (1, 0),   // up
        (0, 1),   // right
        (-1, 0),  // down
        (0, -1),  // left
        (1, 1),   // up-right
        (-1, 1),  // down-right
        (-1, -1), // down-left
        (1, -1),  // up-left
    ];

    static ref XMAS: Vec<char> = vec!['X', 'M', 'A', 'S'];

    static ref CORNER_DIRECTION: Vec<(i32, i32)> = vec![
        (-1, -1), // left up
        (-1, 1),  // right up
        (1, -1),  // left down
        (1, 1),   // right down
    ];

    static ref POSSIBLE_CROSS_MAS: Vec<Vec<char>> = vec![
        vec!['M', 'M', 'S', 'S'],
        vec!['S', 'S', 'M', 'M'],
        vec!['M', 'S', 'M', 'S'],
        vec!['S', 'M', 'S', 'M'],
    ];
}

fn check_xmas(y: i32, x: i32, dx: i32, dy: i32, map: &Vec<Vec<char>>) -> bool {
    let mut y = y;
    let mut x = x;
    let mut i = 0;

    while i < XMAS.len() {
        // range check
        if y < 0 || y >= map.len() as i32 || x < 0 || x >= map[y as usize].len() as i32 {
            return false;
        }

        if XMAS[i] != map[y as usize][x as usize] {
            return false;
        }

        i += 1;
        y += dy;
        x += dx;
    }

    true
}

fn part_one(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut total = 0;
    let mut map: Vec<Vec<char>> = Vec::with_capacity(140);

    for line in reader.lines() {
        let line: String = line.unwrap();
        map.push(line.chars().collect());
    }

    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'X' {
                for (dx, dy) in DIRECTION_LIST.iter() {
                    if check_xmas(y as i32, x as i32, *dx, *dy, &map) {
                        total += 1;
                    }
                }
            }
        }
    }

    println!("PART ONE: {}", total);
}

fn check_cross_mas(y: i32, x: i32, map: &Vec<Vec<char>>) -> bool {
    let mut corner_chars = Vec::with_capacity(4);

    for i in 0..CORNER_DIRECTION.len() {
        let (dy, dx) = CORNER_DIRECTION[i];
        let cy = y + dy;
        let cx = x + dx;

        // range check
        if cy < 0 || cy >= map.len() as i32 || cx < 0 || cx >= map[cy as usize].len() as i32 {
            return false;
        }

        corner_chars.push(map[cy as usize][cx as usize]);
    }

    POSSIBLE_CROSS_MAS.contains(&corner_chars)
}

fn part_two(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut total = 0;
    let mut map: Vec<Vec<char>> = Vec::with_capacity(140);

    for line in reader.lines() {
        let line: String = line.unwrap();
        map.push(line.chars().collect());
    }

    let x_edge = map[0].len() - 1;

    for (y, line) in map.iter().enumerate().skip(1) {
        for (x, c) in line.iter().enumerate().skip(1) {
            if x < x_edge && *c == 'A' {
                if check_cross_mas(y as i32, x as i32, &map) {
                    total += 1;
                }
            }
        }
    }

    println!("PART TWO: {}", total);
}

fn main() {
    let mut total = Duration::new(0, 0);
    for _ in 0..2000 {
        let start = Instant::now();
        part_one("input");
        part_two("input");

        let duration = start.elapsed();
        total += duration;
    }
    println!("Average Time elapsed: {:?}", total / 2000);
}
