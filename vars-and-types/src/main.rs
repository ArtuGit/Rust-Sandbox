fn main() {
    // Variables declaration with explicit type annotation
    let age: u32 = 30;
    let name: &str = "Alice";
    // name = "Bob"; // This is not allowed because name is immutable

    let word = String::from("Hello, World!");
    // word = String::from("Hello, Rust!"); // This is not allowed because word is immutable


    let is_student: bool = false;
    let weight: f64 = 68.5;
    // weight = 70.0; // This is not allowed because weight is immutable

    // Variables declaration with type inference
    let height = 175.3; // Compiler infers f64 type
    let num_students = 100; // Compiler infers i32 type
    let message = "Hello, World!"; // Compiler infers &str type

    // Mutable variable declaration with explicit type annotation
    let mut count: u8 = 0;
    println!("Count: {}", count);
    count = 5; // This is allowed because count is mutable
    let mut word1 = String::from("Hello, World!");
    word1.push_str(" Rust!"); // This is allowed because word1 is mutable

    // Mutable variable declaration with type inference
    let mut temperature = 25.5; // Compiler infers f64 type
    println!("Temperature: {}", temperature);
    temperature = 30.0; // This is allowed because temperature is mutable

    // Shadowing variables
    let x = 5;
    let x = x + 1; // This shadows the previous value of x
    let x = x * 2; // This shadows the previous value of x again

    // Printing variables
    println!("Age: {}", age);
    println!("Name: {}", name);
    println!("Is student: {}", is_student);
    println!("Weight: {}", weight);
    println!("Height: {}", height);
    println!("Number of students: {}", num_students);
    println!("Message: {}", message);
    println!("Count: {}", count);
    println!("Temperature: {}", temperature);
    println!("Value of x: {}", x);
    println!("Word: {}", word);
    println!("Word1: {}", word1);
}