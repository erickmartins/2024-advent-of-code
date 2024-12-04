use std::fs;
use std::io;
use std::cmp;

fn main() {
    let finds = puzzle1().unwrap();
    println!("solution part 1: {finds}");
}

fn puzzle1() -> io::Result<usize>{
    let mut finds = 0;
    let input = fs::read_to_string("input.txt")?;
    let input_lines = input.lines();
    let line_array:Vec<String> = input_lines.map(String::from).collect();
    for i in 0..line_array.len(){
        let line = line_array[i].clone();
        for j in 0..line.len(){
            let curr_char = line.chars().nth(j).unwrap();
            if curr_char == 'X'{
                let is_xmas = lookup(i, j, line_array.clone(), 'X', 5).unwrap();
                finds = finds + is_xmas;
            }
        }
    }

    return Ok(finds);
}


fn lookup(i:usize, j:usize, line_array:Vec<String>, curr_char:char, direction:usize) -> io::Result<usize>{
    let mut xmas = 0;
    let n_lines = line_array.len();
    let linesize = line_array[0].len();
    let min_i = cmp::max((i as isize)-1, 0)as usize;
    let min_j = cmp::max((j as isize)-1, 0)as usize;
    let max_i = cmp::min(i+2, n_lines);
    let max_j = cmp::min(j+2, linesize);
    let next_char = nextchar(curr_char);
    for this_i in min_i..max_i{
        for this_j in min_j..max_j{
            let curr_char = line_array[this_i].chars().nth(this_j).unwrap();
            if curr_char == next_char{
                let this_direction = 9 - (3 * (1 + i - this_i)) - (1 + j - this_j);
                if (direction == 5) || (this_direction == direction){
                    if next_char == 'S'{
                        return Ok(1);
                    }
                    let is_xmas = lookup(this_i, this_j, line_array.clone(), curr_char, this_direction).unwrap();
                    xmas = xmas + is_xmas;
                }
                
            }
        }
    }
    return Ok(xmas);
}

fn nextchar(curr_char:char) -> char{
    let mut next_char:char = 'X';
    if curr_char == 'X' {
        next_char = 'M';
    }else if curr_char == 'M' {
        next_char = 'A';
    }else if curr_char == 'A' {
        next_char = 'S';
    }else if curr_char == 'S' {
        next_char = 'X';
    }
    return next_char;
}