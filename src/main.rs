use core::num;
use std::io;

fn main() {
    println!("Enter a weight in pounds to see what plates you would need to load:");

    let user_input = match get_user_input() {
        Ok(input) => input,
        Err(error) => {
            eprintln!("Error getting user input: {error}");
            return;
        }
    };

    let weight = match parse_user_input(user_input) {
        Ok(weight) => weight,
        Err(error) => {
            eprintln!("Error parsing user input: {error}");
            return;
        }
    };

    let plate_sizes = [45.0, 35.0, 25.0, 10.0, 5.0, 2.5];
    let bar_weight = 45.0;

    if let Err(message) = validate_weight(weight, bar_weight) {
        eprintln!("{message}");
        return;
    }

    if weight == bar_weight {
        println!("You are just doing the bar weight. Go drink some chocolate milk.");
        return;
    }

    let mut weight_without_bar = weight - bar_weight;

    for x in &plate_sizes {
        weight_without_bar = check_plates_per_side(weight_without_bar, *x);
    }
}

fn get_user_input() -> Result<String, io::Error> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    Ok(input)
}

fn parse_user_input(input: String) -> Result<f64, num::ParseFloatError> {
    let weight: f64 = input.trim().parse()?;

    Ok(weight)
}

fn validate_weight(weight: f64, bar_weight: f64) -> Result<(), &'static str> {
    if weight % 5.0 != 0.0 {
        return Err("Weight should be divisible by 5.");
    }

    if weight < bar_weight {
        return Err("Weight should be more than just the bar weight.");
    }

    Ok(())
}

fn check_plates_per_side(weight: f64, plate: f64) -> f64 {
    if weight < (plate * 2.0) {
        weight
    } else {
        let weight_per_side = weight / 2.0;
        let extra_weight_per_side = weight_per_side % plate;
        let plate_weight_per_side = weight_per_side - extra_weight_per_side;
        let plates_per_side = plate_weight_per_side / plate;
        println!("You will need {plates_per_side}x {plate}-lbs plate(s) per side.");
        extra_weight_per_side * 2.0
    }
}
