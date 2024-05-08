use std::io;

fn main() {
    println!("Simple Rust Calculator");
    println!("----------------------");
    println!("Enter expressions like '2 + 3', '10 * 5', etc.");

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

    if parts.len() != 3 {
        return Err("Invalid expression format".to_string());
    }

    let first = parts[0].parse::<f64>();
    let second = parts[2].parse::<f64>();

    match (first, second) {
        (Ok(a), Ok(b)) => {
            let operator = parts[1];
            match operator {
                "+" => Ok(a + b),
                "-" => Ok(a - b),
                "*" => Ok(a * b),
                "/" => {
                    if b == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(a / b)
                    }
                }
                _ => Err("Invalid operator".to_string()),
            }
        }
        _ => Err("Invalid numbers".to_string()),
    }
}
