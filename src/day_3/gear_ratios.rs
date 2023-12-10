use std::io::BufRead;
use crate::helpers::helpers;

pub fn answers() {
    println!("-- Day Three Answers --");
    let part_one = part_one();
    let part_two = part_two();

    println!("| Part one: {} |", part_one);
    println!("| Part two: {} |", part_two);
}

fn part_one() -> i32 {
    let matrice = get_matrice();

    let mut parts_sum: i32 = 0;

    for(row_x, row) in matrice.iter().enumerate(){
        let mut is_previous_digit = false;
        let mut is_connected = false;
        let mut curr_digit: i32 = 0;

        for(char_y, &character) in row.iter().enumerate(){
            if character.is_digit(10) {
                if is_previous_digit {
                    curr_digit = format!("{}{}", curr_digit, character).parse().unwrap();
                } else {
                    curr_digit = character.to_digit(10).unwrap() as i32;
                }

                if !is_connected {
                    is_connected = check_is_connected(row_x, char_y, matrice.clone());
                }

                is_previous_digit = true;
            } else {
                if is_connected {
                    parts_sum += curr_digit;
                }

                is_previous_digit = false;
                is_connected = false;
                curr_digit = 0;
            }
        }

        if is_connected {
            parts_sum += curr_digit;
        }
    }

    return parts_sum;
}

fn check_is_connected(mut row_x: usize, char_y: usize, matrice: Vec<Vec<char>>) -> bool {
    let max_x = 140;
    let max_y = 140;

    let x = row_x as i32;
    let y = char_y as i32;

    let x_arr= [x-1, x, x+1];
    let y_arr = [y-1, y, y+1];

    for x in x_arr {
        if x < 0 || x > max_x - 1 {
            continue
        }

        for y in y_arr {
            if y < 0 || y > max_y -1 {
                continue
            }

            let char = matrice[x as usize][y as usize];
            if !char.is_digit(10) && char != '.' {
                return true;
            }
        }
    }

    return false;
}

fn part_two() -> i32 {
    // Must finish this exercise");
    let matrice = get_matrice();

    let mut gear_ratio_sum: i32 = 0;

    for(row_x, row) in matrice.iter().enumerate(){
        for(char_y, &character) in row.iter().enumerate(){
            if character == '*' {
                gear_ratio_sum += get_gear_ratio(row_x, char_y, matrice.clone());
            }
        }
    }

    return gear_ratio_sum;
}

fn get_gear_ratio(row_x: usize, char_y: usize, matrice: Vec<Vec<char>>) -> i32 {
    let max_x = 140;
    let max_y = 140;

    let x = row_x as i32;
    let y = char_y as i32;

    let x_arr= [x-1, x, x+1];
    let y_arr = [y-1, y, y+1];

    let mut first_val = 0;
    let mut second_val = 0;

    for x in x_arr {
        if x < 0 || x > max_x - 1 {
            continue
        }

        for y in y_arr {
            if y < 0 || y > max_y -1 {
                continue
            }

            let char = matrice[x as usize][y as usize];
            if char.is_digit(10) {
                if first_val == 0 {
                    first_val = get_enclosing_digits(x as usize, y as usize, &matrice, char.to_digit(10).unwrap() as i32);
                }
                let sec = get_enclosing_digits(x as usize, y as usize, &matrice, char.to_digit(10).unwrap() as i32);
                if sec != second_val {
                    second_val = sec;
                }
            }
        }
    }

    return if first_val != second_val {
        first_val * second_val
    } else {
        0
    }
}

fn get_enclosing_digits(x: usize, y: usize, matrice: &Vec<Vec<char>>, curr_val: i32) -> i32 {

    let prev_digits = get_prev_digits(x, y - 1, matrice, None);
    let next_digits = get_next_digits(x, y + 1, matrice, None);

    let mut final_val = format!("{}", curr_val);
    if prev_digits.is_some() {
        final_val = format!("{:?}{}", prev_digits.unwrap(), final_val);
    }

    if next_digits.is_some() {
        final_val = format!("{}{:?}", final_val, next_digits.unwrap());
    }

    return final_val.parse::<i32>().unwrap();
}

fn get_prev_digits(x: usize, y: usize, matrice: &Vec<Vec<char>>, mut curr_val: Option<i32>) -> Option<i32> {
    let char = matrice[x][y];

    if !char.is_digit(10) {
        return curr_val;
    }

    let mut new_curr_val= format!("{}", char).parse::<i32>().unwrap();

    if curr_val.is_some() {
        new_curr_val= format!("{}{}", char, curr_val.unwrap()).parse::<i32>().unwrap();
    }

    if y > 0 {
        return get_prev_digits(x, y - 1, matrice, Some(new_curr_val));
    }

    return Some(new_curr_val);
}

fn get_next_digits(x: usize, y: usize, matrice: &Vec<Vec<char>>, curr_val: Option<i32>) -> Option<i32> {
    let max_y = 10;

    let char = matrice[x][y];

    if !char.is_digit(10) {
        return curr_val;
    }

    let mut new_curr_val= format!("{}", char).parse::<i32>().unwrap();

    if curr_val.is_some() {
        new_curr_val= format!("{}{}", curr_val.unwrap(), char).parse::<i32>().unwrap();
    }

    if y+1 <= max_y - 1 {
        return get_next_digits(x, y + 1, matrice, Some(new_curr_val));
    }

    return Some(new_curr_val);
}

fn get_matrice() -> Vec<Vec<char>> {
    let reader = helpers::get_file_reader("day_3");
    let mut matrice = Vec::new();

    for line in reader.lines() {
        let currline = match line {
            Ok(line) => line,
            Err(_) => panic!("Line is empty"),
        };

        matrice.push(currline.chars().collect::<Vec<char>>());
    }

    return matrice;
}
