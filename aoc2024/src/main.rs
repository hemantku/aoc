use std::{collections::HashMap, fs::read_to_string};

fn part1(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
        left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part2(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut frequency_map = HashMap::new();
    let mut similarity_score = 0;

    for &loc in right.iter() {
        *frequency_map.entry(loc).or_insert(0) += 1;
    }

    for loc in left.iter() {
        match frequency_map.get(loc) {
            Some(frequency) => similarity_score += loc * frequency,
            None => {} 
        }
    }
    similarity_score
}


fn read_input(path: &str) -> (Vec<i32>, Vec<i32>) {
    let content = read_to_string(path).unwrap();
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in content.lines() {
       let location: Vec<i32> = line.split_whitespace()
       .filter_map(|s| s.parse::<i32>().ok())
       .collect();
        
        if location.len() == 2 {
            left.push(location[0]);
            right.push(location[1]);
        }
    }

    left.sort();
    right.sort();

    (left, right)
}

fn main() {
    let path = "/Users/vgh-personal/aoc/aoc2024/src/input.txt";
    let (left, right) = read_input(path);
    let solution_1 = part1(&left, &right);
    println!("Part A: {:?}", solution_1);
    let solution_2 = part2(&left, &right);
    println!("Part B: {:?}", solution_2);
}


