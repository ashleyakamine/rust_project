use std::io;

fn main() {
    println!("Simple Rust Calculator");
    println!("----------------------");

    loop {
        let mut input = String::new();

        println!("Enter expression (or 'exit' to quit):");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim whitespace from input
        let input = input.trim();

        if input == "exit" {
            println!("Goodbye!");
            break;
        }

        let result = evaluate_expression(input);

        match result {
            Ok(value) => println!("Result: {}", value),
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn evaluate_expression(expression: &str) -> Result<f64, String> {
    let parts: Vec<&str> = expression.split_whitespace().collect();

    match parts.len() {
        3 => {
            let first = parts[0].parse::<f64>();
            let second = parts[2].parse::<f64>();

            match (first, second) {
                (Ok(a), Ok(b)) => {
                    match parts[1] {
                        "+" => Ok(a + b),
                        "-" => Ok(a - b),
                        "*" => Ok(a * b),
                        "%" => Ok(a % b),
                        "^" => Ok(a.powf(b)),
                        "/" => {
                            if b == 0.0 {
                                Err("Division by zero".to_string())
                            } else {
                                Ok(a / b)
                            }
                        },
                        _ => Err("Invalid operator".to_string()),
                    }
                }
                _ => Err("Invalid numbers".to_string()),
            }
        }
        2 => {
            let number = parts[1].parse::<f64>();

            match number {
                Ok(a) if parts[0] == "sqrt" => Ok(a.sqrt()),
                Ok(a) if parts[0] == "sin" => Ok(a.sin()),
                Ok(a) if parts[0] == "cos" => Ok(a.cos()),
                Ok(a) if parts[0] == "tan" => Ok(a.tan()),
                _ => Err("Invalid expression format".to_string()),
            }
        }
        _ => Err("Invalid expression format".to_string()),
    }
}
