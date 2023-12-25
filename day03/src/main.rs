use std::collections::btree_map::Range;

use utils::open_file;

#[derive(Copy,Clone)]
struct PartNum {
    num:i32,
    x:i32,
    y:i32,
    size:i32,
    valid:bool
}

impl Default for PartNum {
    fn default() -> Self {
        PartNum {
            num : 0,
            x : -1,
            y : -1,
            size : -1,
            valid : false
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct PartValidator {
    ch:char,
    x:i32,
    y:i32
}

impl Default for PartValidator {
    fn default() -> Self {
        PartValidator {
            ch : ')',
            x : -1,
            y : -1
        }
    }
}

fn print_all_unique_chars(input:String){
    let mut unique_ch = String::new();
    for line in input.lines(){
        for ch in line.chars(){
            if !unique_ch.contains(ch){
                unique_ch.push(ch);
                println!("{}", unique_ch);
            }
        }
    }
    println!("All the unique chars are {}", unique_ch);
}

fn validate_parts (input:String) -> Vec<PartNum> {
    let  (mut parts, validators) = process_parts_and_validators(input);
    parts = match_parts_to_val(parts, validators);
    return parts
}

fn process_parts_and_validators (input:String) -> (Vec<PartNum>, Vec<PartValidator>){
    let mut parts = Vec::new();
    let mut validators = Vec::new();
    let mut temp_part = PartNum::default();
    let mut temp_str = String::new();

    for (y, line) in input.lines().enumerate(){
        for (x,ch) in line.chars().enumerate() {
            if ch == '.' {
                if temp_str.is_empty(){
                    // Intentionally left empty
                }
                else {
                    temp_part.num = temp_str.parse().unwrap();
                    parts.push(temp_part);
                    temp_str.clear();
                    temp_part = PartNum::default();
                }
            }
            else if ch.is_alphanumeric() {
                if temp_str.is_empty() {
                    temp_str.push(ch);
                    temp_part.x = x as i32;
                    temp_part.y = y as i32;
                    temp_part.size = 1;
                }
                else {
                    temp_str.push(ch);
                    temp_part.size += 1;
                }
            }
            else {
                let validator = PartValidator {
                    x : x as i32,
                    y : y as i32,
                    ch : ch
                };
                validators.push(validator);
                }
            }
        }
    return (parts, validators)
}

fn match_parts_to_val(parts:Vec<PartNum>, valids:Vec<PartValidator>) -> Vec<PartNum> {
    let mut matched_parts = Vec::new();
    for mut part in parts.into_iter() {
        for validator in valids.iter() {
            if (part.x - 1..part.x + part.size).contains(&validator.x) && (part.y - 1 .. part.y + 1).contains(&validator.y) {
                part.valid = true;
                println!("Part Valid! {}, x:{}, y:{}, size:{}, and is {}", part.num, part.x, part.y, part.size, part.valid);
                println!("Validator was {}, x:{}, y:{}", validator.ch, validator.x, validator.y)
            }
            //println!("Part {}, x:{}, y:{}, size:{}, and is {}", part.num, part.x, part.y, part.size, part.valid);
        }
        matched_parts.push(part);
    }
    return matched_parts
}

fn filter_parts (input:Vec<PartNum>) -> Vec<i32> {
    let mut num_vec = Vec::new();
    for part in input.iter() {
        println!("Part {}, x:{}, y:{}, size:{}, and is {}", part.num, part.x, part.y, part.size, part.valid);
        if part.valid {
            num_vec.push(part.num);
        }
    }
    return num_vec
}


fn main() {
    // Windows was used to run this code so it, regrettably, uses NT style paths.
    let contents = open_file("day03\\src\\input_test.txt");


    if let Ok(foo) = contents {
        let valid_parts = validate_parts(foo);
        let valid_nums = filter_parts(valid_parts);
        let sum:i32 = valid_nums.iter().sum();
        println!("Parts sum is {}", sum);
    }
    else {
        println!("no");
    }
}
