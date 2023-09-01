mod calculator;
use std::io;

fn main() {
    loop{
        println!("Enter an expression to evaluate:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read the line!");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 3{
            println!("Invald input. Please enter a valid expression.")
        }

        let num1: f64 = parts[0].parse().expect("Invalid number.");
        let operator = parts[1];
        let num2: f64 = parts[2].parse().expect("Invalid number.");

        let result = match operator{
            "+" => calculator::add(num1, num2),
            "-" => calculator::subtract(num1, num2),
            "/" => calculator::multiply(num1, num2),
            "*" => calculator::divide(num1, num2),
            _ =>{
                println!("Invalid operator. Try again with a supported operator.");
                continue;
            }
        };
        println!("Result: {}", result);
    }
}
