use std::fs;
use std::io;
use regex::Regex;


fn main() {
    let mults = puzzle1().unwrap();
    println!("solution part 1: {mults}");
    let mults_en = puzzle2().unwrap();
    println!("solution part 2: {mults_en}");
}

fn puzzle1() -> io::Result<i32> {
    let mut mults:i32 = 0;
    let input = fs::read_to_string("input.txt")?; 
    let input_str = input.as_str();   
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    for (_, [first, second]) in re.captures_iter(input_str).map(|caps| caps.extract()){
        let first_val = first.parse::<i32>().unwrap();
        let second_val = second.parse::<i32>().unwrap();
        mults = mults + (first_val * second_val);
    }
    
    return Ok(mults);
}

fn puzzle2() -> io::Result<i32> {
    let mut mults:i32 = 0;
    let input = fs::read_to_string("input.txt")?; 
    let input_str = input.as_str();
    let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))|(don\'t\(\))").unwrap();
    let sub_re = Regex::new(r"([0-9]{1,3}),([0-9]{1,3})").unwrap();
    let mut on = true;

    for (_, [ext_str]) in re.captures_iter(input_str).map(|caps| caps.extract()){
        dbg!(ext_str);
        if ext_str == "do()" {
            on = true;
        }else if ext_str == "don't()"{
            on = false;
        }else if on{
            let caps = sub_re.captures(ext_str).unwrap();
            let first_val = &caps[1].parse::<i32>().unwrap();
            let second_val = &caps[2].parse::<i32>().unwrap();
            mults = mults + (first_val * second_val);
        }
    }

    
    return Ok(mults);
}
