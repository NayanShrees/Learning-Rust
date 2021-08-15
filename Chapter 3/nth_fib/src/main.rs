use std::io;

fn main() {
    let n = loop {
        println!("Enter a number to return the nth fibonacci number:");
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        match n.trim().parse::<u32>() {
            Ok(n) => break n,
            Err(_) => {}
        };
    };

    println!("{}", fib(n));
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
