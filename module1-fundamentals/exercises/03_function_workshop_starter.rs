fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }

    for i in 2..=((number as f64).sqrt() as u32) {
        if number % i == 0 {
            return false;
        }
    }

    true
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    let sum1 = add(10, 25);
    let sum2 = add(7, 3);
    println!("Sum of 10 and 25 is: {}", sum1);
    println!("Sum of 7 and 3 is: {}", sum2);

    let area1 = calculate_rectangle_area(5.0, 10.0);
    let area2 = calculate_rectangle_area(3.5, 2.0);
    println!("Area of rectangle with width 5 and height 10 is: {} square units", area1);
    println!("Area of rectangle with width 3.5 and height 2 is: {} square units", area2);

    let prime_check1 = is_prime(7);
    let prime_check2 = is_prime(12);
    println!("Is 7 a prime number? {}", prime_check1);
    println!("Is 12 a prime number? {}", prime_check2);

    let celsius1 = fahrenheit_to_celsius(98.6);
    let celsius2 = fahrenheit_to_celsius(32.0);
    println!("98.6°F is equivalent to {:.1}°C", celsius1);
    println!("32.0°F is equivalent to {:.1}°C", celsius2);
}