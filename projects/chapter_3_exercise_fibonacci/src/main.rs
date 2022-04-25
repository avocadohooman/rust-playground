use std::io;

fn main() {
    println!("Generate the nth Fibonacci number. Provider a number:");

    loop {
        let mut nth_fibonacci = String::new();

        io::stdin()
            .read_line(&mut nth_fibonacci)
            .expect("Failed reading input");
        let nth_fibonacci: u32 = match nth_fibonacci.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Number must be >= 1");
                continue;
            }
        };

        println!(
            "The {} fibonacci number is: {}",
            nth_fibonacci,
            fib(nth_fibonacci)
        );
        break;
    }
}

fn fib(nth_fibonacci: u32) -> u128 {
    if nth_fibonacci == 0 {
        return 0;
    } else if nth_fibonacci == 1 {
        return nth_fibonacci as u128;
    }
    fib(nth_fibonacci - 1) + fib(nth_fibonacci - 2)
}
