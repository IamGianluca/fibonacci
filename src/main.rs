use std::io;
use std::io::Write;

fn main() {
    loop {
        print!("What number in the Fibonacci series do you want to retrieve? ");
        io::stdout().flush().unwrap();

        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: u64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        let result: u64 = fibonacci(number);
        println!("The {number}th number in the Fibonacci series is {result}");
    }
}

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
