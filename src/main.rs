use std::io;

fn main() {
    println!("Enter a weight in pounds to see what plates you would need to load:");

    let weight = get_user_input();

    if let Err(message) = validate_weight(weight) {
        eprintln!("{message}");
    }
}

fn get_user_input() -> i32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let weight: i32 = input.trim().parse().expect("Not a valid number!");

    weight
}

fn validate_weight(weight: i32) -> Result<(), &'static str> {
    if weight % 5 != 0 {
        eprintln!("Weight should be divisible by 5.");
    }

    Ok(())
}
