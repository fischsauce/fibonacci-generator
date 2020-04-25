use std::io;

fn main() {
 
    loop {
        println!("Hello user, welcome to Fibonacci's world!\n\nto begin, just type any number...");

        let mut fib2: i64 = 0;
        let mut fib1: i64 = 1;

        let mut nth = String::new();
        io::stdin().read_line(&mut nth).expect("Failed to read line");
        let mut nth: i64 = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue
            },
        };

        if nth == 1 {
            println!("\n--------------------------------\nThe {}th Fibonacci number is: 0\n--------------------------------\n", nth);
        } else if nth == 2 {
            println!("\n--------------------------------\nThe {}th Fibonacci number is: 1\n--------------------------------\n", nth);
        } else {
            for n in 1..(nth -1) {   

                let fibonacci = fib1 + fib2;

                fib2 = fib1;
                fib1 = fibonacci;
                
                if n == (nth -2) {
                    println!("\n-----------------------------------\nThe {}th Fibonacci number is: {}\n-----------------------------------\n", nth, fibonacci);
                }
            };
        };
    }
}
