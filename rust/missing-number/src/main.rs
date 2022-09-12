use std::io;
use std::process::exit;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
fn main() {
    let mut string_input = String::new();
    io::stdin()
        .read_line(&mut string_input)
        .expect("failed to read stdin");
    let n: usize = string_input.trim().parse().expect("Expected number");
    string_input = String::new();
    io::stdin()
        .read_line(&mut string_input)
        .expect("failed to read stdin");
    let mut other = string_input
        .trim()
        .split(" ")
        .map(|letter| letter.trim().parse::<usize>().expect("expected a number!"))
        .collect::<Vec<usize>>();
    if n == 2 {
        if other[0] == 2 {
            print!("1")
        } else {
            print!("2");
        }
        exit(0);
    }
    other.sort();
    let other = Arc::new(other);

    let thread_n = std::cmp::min(
        32,
        n/2,
    );
    let delta = (other.len() + thread_n - 1) / thread_n;
    let mut join_handlers: Vec<JoinHandle<()>> = vec![];
    for i in 0..thread_n {
        let other = other.clone();
        let i = i.clone();
        let handle = thread::spawn(move || {
            let  start = i * delta;
            let mut end = (i + 1) * delta;
            if i == thread_n - 1 {
                end = other.len() + 1 ;
            }
            for j in start+1..end+1 {
                if other.binary_search(&j).is_err() {
                    print!("{}", j);
                    exit(0);
                }
                // match other.binary_search(&j) {
                //     Ok(_) => {
                //     }
                //     Err(_) => {
                //         print!("{}", j);
                //         exit(0);
                //     }
                // }
            }
        });

        join_handlers.push(handle);
    }

    for handler in join_handlers {
        handler.join().unwrap();
    }


}
