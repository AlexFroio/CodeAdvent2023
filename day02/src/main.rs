use core::fmt;

use utils::open_file;
use regex::Regex;

struct Bag {
    id: i32,
    rounds: Vec<Round>,
    valid: bool
}

struct Round {
    red:i32,
    green:i32,
    blue:i32
}

impl Default for Bag {
    fn default() -> Self {
        Bag {
            id: -1,
            rounds: Vec::new(),
            valid: true
        }
    }
}

impl Default for Round {
    fn default() -> Self {
        Round {
            red: 0,
            green: 0,
            blue: 0
        }
    }
}

impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "G{}, R{}, B{}", self.green, self.red, self.blue)
    }
}

fn sum_possible(default:Round, input:String) -> Vec<i32> {
    let mut poss_games = Vec::new();
    let re = Regex::new(r"[:]+").unwrap();
    let re2 = Regex::new(r"[;]").unwrap();
    let re3 = Regex::new(r"[,]").unwrap();
    for line in input.lines() {
        let mut bag = Bag::default();
        for split_1 in re.split(line) {
            if split_1.contains("Game"){
                let res = split_1.replace("Game ", "");
                bag.id = res.parse::<i32>().unwrap();
                println!("{}", bag.id);
            }
            else {
                for split_2 in re2.split(split_1){
                    let mut round = Round::default();
                    for split_3 in re3.split(split_2){
                        if split_3.contains("green"){
                            let mut res = split_3.replace(" ", "");
                            res = res.replace("green", "");
                            round.green += res.parse::<i32>().unwrap();
                        }
                        else if split_3.contains("red"){
                            let mut res = split_3.replace(" ", "");
                            res = res.replace("red", "");
                            round.red += res.parse::<i32>().unwrap();
                        }
                        else if split_3.contains("blue"){
                            let mut res = split_3.replace("blue", "");
                            res = res.replace(" ", "");
                            round.blue += res.parse::<i32>().unwrap();
                        }
                    }
                    println!("Round has {}", round);
                    bag.rounds.push(round);
                }
            }
            for round in bag.rounds.iter() {
                if (default.green < round.green) | (default.red < round.red) | (default.blue < round.blue){
                    bag.valid = false;
                }
            }
        }
        if bag.valid {
            poss_games.push(bag.id);
        }
    }
    return poss_games;
}

fn main() {
    // Windows was used to run this code so it, regrettably, uses NT style paths.
    let contents = open_file("day02\\src\\input.txt");
    let possible = Round {
        red: 12,
        green: 13,
        blue: 14
    };
    if let Ok(foo) = contents {
        let games_possible = sum_possible(possible, foo);
        println!("Valid games have IDs: {:?}", games_possible);
        let sum:i32 = games_possible.iter().sum();
        println!("Possible games ID sum is {}", sum);
    }
    else {
        println!("no");
    }
}
