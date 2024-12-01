mod puzzle1;
mod puzzle2;

fn main() {
    // --snip--
    let distance = puzzle1::puzzle1().unwrap();
    println!("solution part 1: {distance}");
    let similarity = puzzle2::puzzle2().unwrap();
    println!("solution part 2: {similarity}");
}