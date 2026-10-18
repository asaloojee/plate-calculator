use std::io;

fn main() {
    println!("Enter a weight in pounds to see what plates you would need to load:");

    let weight = get_user_input() as f64;
    let plate_sizes = [45.0, 35.0, 25.0, 10.0, 5.0, 2.5];
    let bar_weight = 45.0;

    if let Err(message) = validate_weight(&weight, &bar_weight) {
        eprintln!("{message}");
    }

    let mut weight_without_bar = weight - bar_weight;

    for x in &plate_sizes {
        weight_without_bar = check_plates_per_side(weight_without_bar, *x);
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

fn validate_weight(weight: &f64, bar_weight: &f64) -> Result<f64, &'static str> {
    if weight % 5.0 != 0.0 {
        eprintln!("Weight should be divisible by 5.");
    }

    if weight < bar_weight {
        eprintln!("Weight should be more than just the bar weight.");
    }

    Ok(*weight)
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
