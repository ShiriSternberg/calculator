//! A program for a calculator

use std::{fmt::Debug, io, str::FromStr};

/// Calculators operations
const ADDITION_OPERATION: char = '+';
const SUBTRACTION_OPERATION: char = '-';
const MULTIPLICATION_OPERATION: char = '*';
const DIVISION_OPERATION: char = '/';

fn main() {
    println!("Rust Calculator");

    println!("Enter first number:");
    let first_number: i32 = read_input("Invalid integer");

    println!("Enter second number:");
    let second_number: i32 = read_input("Invalid integer");

    println!("Choose operation (+, -, *, /):");
    let operation: char = read_input("Invalid character");

    println!(
        "Result: {}",
        calculate(first_number, second_number, operation)
    );
}

// Calculating the result of the received math problem.
fn calculate(first_number: i32, second_number: i32, operation: char) -> i32 {
    match operation {
        ADDITION_OPERATION => first_number
            .checked_add(second_number)
            .expect("Addition failed"),
        SUBTRACTION_OPERATION => first_number
            .checked_sub(second_number)
            .expect("Subtraction failed"),
        MULTIPLICATION_OPERATION => first_number
            .checked_mul(second_number)
            .expect("Multiplication failed"),
        DIVISION_OPERATION => first_number
            .checked_div(second_number)
            .expect("Division failed"),
        _ => panic!("Invalid operation"),
    }
}

// Receives and returns the user's input.
fn receive_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

// Receives input from the user and parse it to the wanted type.
// Receives an error to raise in case the function fails.
fn read_input<T: FromStr>(error: &str) -> T
where
    T::Err: Debug,
{
    receive_input().trim().parse().expect(error)
}
