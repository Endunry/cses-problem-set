use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read stdin");    
    let input: u64 = input.trim().parse().expect("Expected a number");
    let mut current = input;
    'mainLoop: loop {
        print!("{} ", current);
        if current == 1 {break 'mainLoop}
        if current % 2 == 0 {
            current = current / 2;
        } else {
            current = current * 3 + 1;
        }
    }
}
