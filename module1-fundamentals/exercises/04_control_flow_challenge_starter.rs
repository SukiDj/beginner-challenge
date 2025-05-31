use std::io;

fn main() {
    println!("=== FizzBuzz Challenge ===");
    
    for i in 1..=20 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }

    println!("\n=== Calculator ===");

    let mut running = true;

    while running {
        println!("\nChoose an operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");
        println!("Enter your choice: ");
        
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        if choice == 5 {
            running = false;
            continue;
        }

        let num1 = read_number("Enter first number: ");
        let num2 = read_number("Enter second number: ");

        match choice {
            1 => println!("Result: {} + {} = {}", num1, num2, num1 + num2),
            2 => println!("Result: {} - {} = {}", num1, num2, num1 - num2),
            3 => println!("Result: {} * {} = {}", num1, num2, num1 * num2),
            4 => {
                if num2 == 0.0 {
                    println!("Error: Cannot divide by zero.");
                } else {
                    println!("Result: {} / {} = {}", num1, num2, num1 / num2);
                }
            }
            _ => println!("Invalid option. Please try again."),
        }

        println!("Do you want to perform another calculation? (y/n): ");
        let mut again = String::new();
        io::stdin()
            .read_line(&mut again)
            .expect("Failed to read line");
        if again.trim().to_lowercase() != "y" {
            running = false;
        }
    }

    println!("Thank you for using the calculator!");
}

fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number. Please try again."),
        }
    }
}
