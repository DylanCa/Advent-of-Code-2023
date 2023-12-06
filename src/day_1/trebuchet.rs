use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn answers() {
    println!("-- Day One Answers --");
    let part_one = part_one();
    let part_two = part_two();

    println!("| Part one: {} |", part_one);
    println!("| Part two: {} |", part_two);
}

pub fn part_one() -> i32 {
    let reader = get_file_reader();

    const RADIX: u32 = 10;
    let mut total = 0;

    for line in reader.lines() {
        let currline = match line {
            Ok(line) => line,
            Err(_) => panic!("Line is empty"),
        };

        let filtered_line: String = currline.chars().filter(|c| c.is_numeric()).collect();
        let num = format!("{:?}{:?}", filtered_line.chars().next().unwrap().to_digit(RADIX).unwrap(), filtered_line.chars().last().unwrap().to_digit(RADIX).unwrap());

        total += num.parse::<i32>().unwrap();
    }

    return total;
}

pub fn part_two() -> i32 {
    let reader = get_file_reader();

    const RADIX: u32 = 10;
    let mut total = 0;

    for line in reader.lines() {
        let mut currline = match line {
            Ok(line) => line,
            Err(_) => panic!("Line is empty"),
        };

        currline = currline.replace("one", "o1ne");
        currline = currline.replace("two", "t2wo");
        currline = currline.replace("three", "th3ree");
        currline = currline.replace("four", "fo4ur");
        currline = currline.replace("five", "fi5ve");
        currline = currline.replace("six", "s6x");
        currline = currline.replace("seven", "se7en");
        currline = currline.replace("eight", "ei8ht");
        currline = currline.replace("nine", "ni9ne");

        let filtered_line: String = currline.chars().filter(|c| c.is_numeric()).collect();
        let num = format!("{:?}{:?}", filtered_line.chars().next().unwrap().to_digit(RADIX).unwrap(), filtered_line.chars().last().unwrap().to_digit(RADIX).unwrap());

        total += num.parse::<i32>().unwrap();
    }

    return total;
}

fn get_file_reader() -> BufReader<File> {
    let file = match File::open("./src/day_1/input.txt") {
        Ok(file) => file,
        Err(err) => panic!("Failed to open file: {}", err),
    };
    return BufReader::new(file);
}