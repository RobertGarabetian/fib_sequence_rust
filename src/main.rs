use std::io;
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failure to except input");
    
    let input: i32 = input.trim().parse().expect("Failure to type cast");
    for int in 0..input {
        println!("fibonacci ({}) => {}", int, fib(int));
    }
}

fn fib (x: i32) -> i32 {
    if x == 0 {
    return 0
    } else if x == 1 {
       return 1
    } else {
       return fib(x-1) + fib(x-2);
    }
}