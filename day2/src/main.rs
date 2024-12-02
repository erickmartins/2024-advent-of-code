use std::fs;
use std::io;

fn main() {
    let safes = puzzle1().unwrap();
    println!("solution part 1: {safes}");
}

fn puzzle1() -> io::Result<usize> {
    let mut safes = 0;

    let input = fs::read_to_string("input.txt")?;
    let input_lines = input.lines();
    let line_array:Vec<String> = input_lines.map(String::from).collect();
    for line in line_array{
        let nums: Vec<i32> = line.split(" ").map(|s| s.parse().unwrap()).collect();
        let differences = &nums.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
        
    }

    return Ok(safes);
}

