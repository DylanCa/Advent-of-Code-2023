use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use crate::helpers::helpers;

pub fn answers() {
    println!("-- Day Four Answers --");
    let part_one = part_one();
    let part_two = part_two();

    println!("| Part one: {} |", part_one);
    println!("| Part two: {} |", part_two);
}

pub fn part_one() -> i32 {
    let reader = helpers::get_file_reader("day_4");
    let mut total = 0;

    for line in reader.lines() {
        let currline = match line {
            Ok(line) => line,
            Err(_) => panic!("Line is empty"),
        };

        let mut splitted_line = currline.split(':').last().unwrap().split('|');
        let nums = splitted_line.next().unwrap();
        let winning_nums: Vec<_> = splitted_line.last().unwrap().split_whitespace().map(|num| num.parse::<i32>().unwrap()).rev().collect();

        let mut score = 0;

        for num in nums.split_whitespace() {
            if winning_nums.contains(&num.parse::<i32>().unwrap()) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }

        total += score;
    }

    return total;
}

pub fn part_two() -> usize {
    let reader = helpers::get_file_reader("day_4");
    let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

    return recursive(&lines, 0, lines.len())
}

/* This could be improved greatly by reverse iterating through the file and storing each entry score
& quick-mathing, for instance:
- line 4 -> 0 pt
- line 3 -> 1 pt
- line 2 -> 1 pt
- line 1 -> 3 pts
And the final result being :
- line 1 called 1 time
- line 2 called 2 times
- line 3 called 3 times
- line 4 called 3 times
without having to recalculate everything each time */
fn recursive(lines: &Vec<String>, x: usize, max_x: usize) -> usize {
    let mut cards: usize = 0;

    for idx in x..max_x {
        cards += 1;
        let line = lines.get(idx);

        let currline = match line {
            Some(line) => line,
            None => panic!("Line is empty"),
        };

        let mut splitted_line = currline.split(':').last().unwrap().split('|');
        let nums = splitted_line.next().unwrap();
        let winning_nums: Vec<_> = splitted_line.last().unwrap().split_whitespace().map(|num| num.parse::<i32>().unwrap()).rev().collect();

        let mut matches: usize = 0;

        for num in nums.split_whitespace() {
            if winning_nums.contains(&num.parse::<i32>().unwrap()) {
                matches += 1
            }
        }

        if matches > 0 {
            cards += recursive(&lines, idx + 1, idx + 1 + matches);
        }
    }

    return cards;
}