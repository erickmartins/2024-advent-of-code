use std::fs;
use std::io;
use regex::Regex;


fn main() {
    let mults = puzzle1().unwrap();
    println!("solution part 1: {mults}");
}

fn puzzle1() -> io::Result<usize> {
    let mut mults = 0;
    // let input = fs::read_to_string("input.txt")?; 
    // let input_str = input.as_str();   
    // let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();
    let hay = "On 2010-03-14, I became a Tenneessee lamb.";
    let (_, [year, month, day]) =re.captures(hay).map(|caps| caps.extract()).unwrap();
    // let (full, [first, second]) = re.captures(input_str).map(|caps| caps.extract()).unwrap();
    dbg!(year);
    return Ok(mults);
}
