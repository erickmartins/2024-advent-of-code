use std::fs;
use std::io;

pub fn puzzle2() -> io::Result<usize> {
    let mut similarity = 0;

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

    for i in 0..first.len(){
        let val = first[i];
        let factor = second.iter().filter(|&n| *n == val).count();
        similarity = similarity + (val * factor);
    }

    return Ok(similarity);
}

