use std::io::BufRead;
use crate::helpers::helpers;

pub fn answers() {
    println!("-- Day Two Answers --");
    let part_one = part_one();
    let part_two = crate::day_2::cube_conundrum::part_two();

    println!("| Part one: {} |", part_one);
    println!("| Part two: {} |", part_two);
}

pub fn part_one() -> i32 {
    let reader = helpers::get_file_reader("day_2");

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut total_valid_game_numbers = 0;

    for line in reader.lines() {
        let currline = match line {
            Ok(line) => line,
            Err(_) => panic!("Line is empty"),
        };

        let mut splitted_line = currline.split(':');
        let game_number = splitted_line.next().unwrap().split_whitespace().last().unwrap().parse::<i32>().unwrap();
        let mut valid_game = true;

        let sets = splitted_line.last().unwrap().split(';');

        for set in sets {
            for draw in set.split(','){
                let mut split_draw = draw.trim().split_whitespace();
                let num = split_draw.next().unwrap().parse::<i32>().unwrap();
                let colour = split_draw.last().unwrap();

                if (colour == "green" && num > max_green) ||
                    (colour == "red" && num > max_red) ||
                    (colour == "blue" && num > max_blue){
                    valid_game = false;
                    break
                }
            }

            if !valid_game{
                break
            }
        }

        if valid_game{
            total_valid_game_numbers += game_number;
        }
    }

    return total_valid_game_numbers;
}

pub fn part_two() -> i32 {
    let mut sum_sets_power = 0;
    let reader = helpers::get_file_reader("day_2");
    for line in reader.lines() {
        let currline = match line {
            Ok(line) => line,
            Err(_) => panic!("Line is empty"),
        };

        let splitted_line = currline.split(':');
        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        let sets = splitted_line.last().unwrap().split(';');

        for set in sets {
            for draw in set.split(','){
                let mut split_draw = draw.trim().split_whitespace();
                let num = split_draw.next().unwrap().parse::<i32>().unwrap();
                let colour = split_draw.last().unwrap();

                if colour == "red" && num > min_red{
                    min_red = num
                }

                if colour == "blue" && num > min_blue{
                    min_blue = num
                }

                if colour == "green" && num > min_green{
                    min_green = num
                }
            }
        }

        let power = min_green * min_red * min_blue;
        sum_sets_power += power;
    }

    return sum_sets_power;
}
