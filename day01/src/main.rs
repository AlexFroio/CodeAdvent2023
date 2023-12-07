use core::fmt;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, AsRefStr};
use utils::open_file;

#[derive(Debug, EnumIter, AsRefStr)]
enum TextToDigit{
    one,
    two,
    three,
    four,
    five,
    six,
    seven,
    eight,
    nine,
}

impl fmt::Display for TextToDigit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl TextToDigit {
    fn value(&self) -> char {
        match *self {
            TextToDigit::one => '1',
            TextToDigit::two => '2',
            TextToDigit::three => '3',
            TextToDigit::four => '4',
            TextToDigit::five => '5',
            TextToDigit::six => '6',
            TextToDigit::seven => '7',
            TextToDigit::eight => '8',
            TextToDigit::nine => '9'
        }
    }
}

fn sum_first_and_last(input:String)-> Vec<i32> {
    let mut cal_nums:Vec<i32> = Vec::new();
    for line in input.lines(){
        println!("Input line:{}",line);
        let mut x:char = 'z';
        let mut y:char = 'z';
        let mut num:String = "".to_owned();
        for ch in line.chars(){
            num.push(ch);
            if ch.is_ascii_digit(){
                if x == 'z'{
                    x = ch;
                }
                else {
                    y = ch;
                }
                num.clear();
            }
            else {
                for number in TextToDigit::iter(){
                    if num.contains(number.as_ref())
                    {
                        if x == 'z'{
                            x = number.value();
                        }
                        else {
                            y = number.value();
                        }
                        //println!("{}, {}, x={}, y ={}", num, number, x, y);
                        num.clear();
                        break;
                    }
                }
            }
        }
        if (x != 'z') & (y == 'z'){
            y = x;
        }
        println!("Out of the loop:{}, x={}, y ={}", num, x, y);
        let digit = format!("{}{}", x, y);
        println!("Final digits are {}", digit);
        cal_nums.push(digit.parse::<i32>().unwrap());
    }

    return cal_nums
}


fn main() {
    // Windows was used to run this code so it, regrettably, uses NT style paths.
    let contents = open_file("day01\\src\\input2.txt");

    if let Ok(foo) = contents {
        let cal_nums = sum_first_and_last(foo);
        let sum:i32 = cal_nums.iter().sum();
        println!("Array length is {} and its values are {:?}", cal_nums.len(), cal_nums);
        println!("Final sum is {} ", sum);
    }
    else {
        println!("no");
    }

}

