use std::io;

fn fibonacci_up_to(var: u64) -> Vec<u64> {
    let mut fib_sequence = vec![0, 1]; // Initialize with the first two Fibonacci numbers
    let mut next_fib = 1;

    while next_fib <= var {
        let new_fib = fib_sequence[fib_sequence.len() - 1] + fib_sequence[fib_sequence.len() - 2];
        fib_sequence.push(new_fib);
        next_fib = new_fib;
    }

    fib_sequence
}


fn main() {
    println!("Please enter a number:");

    // Create a mutable String to store the input
    let mut input = String::new();

    // Read input from the console
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    // Parse the input string into an integer
    let number: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Not a valid number!");
            return;
        }
    };

    println!("You entered: {}", number);
    let fibonacci_sequence = fibonacci_up_to(number);

    println!("Fibonacci sequence up to {}: {:?}", number, fibonacci_sequence);
}