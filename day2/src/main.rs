use std::fs;
use std::io;

fn main() {
    let safes = puzzle1().unwrap();
    println!("solution part 1: {safes}");
    let moresafes = puzzle2().unwrap();
    println!("solution part 2: {moresafes}");
}

fn puzzle1() -> io::Result<usize> {
    let mut safes = 0;
    let input = fs::read_to_string("input.txt")?;
    let input_lines = input.lines();
    let line_array:Vec<String> = input_lines.map(String::from).collect();
    for line in line_array{
        let nums: Vec<i32> = line.split(" ").map(|s| s.parse().unwrap()).collect();
        let differences = &nums.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
        let max_diffs = differences.iter().map(|x| x.abs()).collect::<Vec<_>>();
        let max_diff = max_diffs.iter().max().unwrap();
        if (differences.iter().all(|&item| item < 0) || differences.iter().all(|&item| item > 0)) && *max_diff <= 3 {
            safes += 1;
        }
        
    }

    return Ok(safes);
}

fn puzzle2() -> io::Result<usize> {
    let mut safes = 0;
    let input = fs::read_to_string("input.txt")?;
    let input_lines = input.lines();
    let line_array:Vec<String> = input_lines.map(String::from).collect();
    for line in line_array{
        let nums: Vec<i32> = line.split(" ").map(|s| s.parse().unwrap()).collect();
        let differences = &nums.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
        let abs_diffs = differences.iter().map(|x| x.abs()).collect::<Vec<_>>();
        let max_diff = abs_diffs.iter().max().unwrap();
        if (differences.iter().all(|&item| item < 0) || differences.iter().all(|&item| item > 0)) && *max_diff <= 3 {
            safes += 1;
        }else{
            for i in 0..nums.len(){
                let mut ersatz = nums.clone();
                ersatz.remove(i);
                let rz_differences = &ersatz.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
                let rz_abs_diffs = rz_differences.iter().map(|x| x.abs()).collect::<Vec<_>>();
                let rz_max_diff = rz_abs_diffs.iter().max().unwrap();
                if (rz_differences.iter().all(|&item| item < 0) || rz_differences.iter().all(|&item| item > 0)) && *rz_max_diff <= 3 {
                    safes += 1;
                    break;
                }
            }
        }
    }

    return Ok(safes);
}

