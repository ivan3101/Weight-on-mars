use std::io;

const GRAVITY_ON_EARTH: f32 = 9.81;
const GRAVITY_ON_MARS: f32 = 3.72076;

fn main() {
    let your_weight = read_user_weight();
    let your_weight_on_mars = calculate_weight_on_mars(your_weight);

    println!("Your weight on Mars is: {} kg", your_weight_on_mars)
}

fn calculate_weight_on_mars(weight_on_earth: f32) -> f32 {
    (weight_on_earth / GRAVITY_ON_EARTH) * GRAVITY_ON_MARS
}

fn read_user_weight() -> f32 {
    let mut input = String::new();

    println!("Please enter your weight in kg:");
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    weight
}