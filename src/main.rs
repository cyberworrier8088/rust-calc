// main.rs

use std::io::{self, Write}; // import standerd input and output

// add func for two number
fn add(a: f64, b: f64) {
    println!("Result : {}", a + b);
}

// sub func for two number
fn sub(a: f64, b: f64) {
    println!("Result : {}", a - b);
}

// mul func for two number
fn mul(a: f64, b: f64) {
    println!("Result : {}", a * b);
}

// div func for two number
fn div(a: f64, b: f64) {
    println!("Result : {}", a / b);
}

// sqr func for one number
fn sqr(a: f64) {
    println!("Result : {}", a.sqrt());
}


fn main() {
    // this for out put  the simple calc txt :)0
    println!("--- Rust simple calculator ---");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");
    println!("5. square (squre only calculate first entering number only. 😔)");
    println!("Type 'exit' to quit.\n");
    print!("Enter your choice: ");

    //this for user input choice :)
    let _ = io::stdout().flush(); // this _ thing for safly progrem. no crash no warning

    let mut choice = String::new();

    let _ = io::stdin().read_line(&mut choice).expect("Error");

    let choice = choice.trim();

    // this for user input number;
    print!("Enter your first number: ");
    let _ = io::stdout().flush();

    let mut num1 = String::new();
    let _ = io::stdin().read_line(&mut num1).expect("Error");

    print!("Enter your second number: ");
    let _ = io::stdout().flush();

    let mut num2 = String::new();
    let _ = io::stdin().read_line(&mut num2).expect("Error");

    let num1: f64 = num1.trim().parse().expect("Inv input");
    let num2: f64 = num2.trim().parse().expect("Inv input");

    // match function to call the func.
    match choice {
        "1" => add(num1, num2),
        "2" => sub(num1, num2),
        "3" => mul(num1, num2),
        "4" => div(num1, num2),
        "5" => sqr(num1),
        "exit" => println!("exit"),      // Exit the program using std
        _ => println!("Invalid choice"), // Invalid choice message
    }
}

// done!
// This made for learning new things :)
// enjoy coding!(^^)/


// i hoping evary one enjoy this rust project