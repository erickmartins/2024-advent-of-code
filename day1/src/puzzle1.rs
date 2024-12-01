use std::fs;
use std::io;

pub fn puzzle1() -> io::Result<usize> {
    let input = fs::read_to_string("puzzle1/input.txt")?;
    let input_lines = input.lines();
    let mut first: Vec<usize>= vec![];
    let mut second: Vec<usize>= vec![];
    let line_array:Vec<String> = input_lines.map(String::from).collect();
    for line in line_array{
        let mut parts = line.splitn(2, "   ");
        first.push(parts.next().unwrap().parse().unwrap());
        second.push(parts.next().unwrap().parse().unwrap());
    }
    first.sort();
    second.sort();
    let mut distance = 0;
    for i in 0..first.len(){
        let val = first[i].abs_diff(second[i]);
        distance = distance + val;
    }

    return Ok(distance);
}

