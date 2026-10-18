use std::io;

fn main() {
    println!("Enter a weight in pounds to see what plates you would need to load:");

    let weight = get_user_input();
    let plate_sizes = [45.0, 35.0, 25.0, 10.0, 5.0, 2.5];

    if let Err(message) = validate_weight(&weight) {
        eprintln!("{message}");
    }

    let weight_without_bar = weight - 45;

    plate_sizes.iter().fold(weight_without_bar, |acc, &plate| {
        check_num_of_plates(acc as f64, plate);
        acc
    });
}

fn get_user_input() -> i32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let weight: i32 = input.trim().parse().expect("Not a valid number!");

    weight
}

fn validate_weight(weight: &i32) -> Result<f64, &'static str> {
    if weight % 5 != 0 {
        eprintln!("Weight should be divisible by 5.");
    }

    Ok(*weight as f64)
}

fn check_num_of_plates(weight: f64, plate: f64) -> f64 {
    if weight % (plate * 2.0) > 0.0 && weight % (plate * 2.0) == 0.0 {
        let num_of_plates = weight / plate;
        println!("You will need {num_of_plates}x {plate} plates.");
        0.0
    } else if weight % (plate * 2.0) > 0.0 && weight % (plate * 2.0) != 0.0 {
        let plates_per_side = (weight / plate) / 2.0;
        println!("You will need {plates_per_side}x {plate} plates per side.");
        weight - plates_per_side * 2.0
    } else {
        weight
    }
}
