mod day1;
mod day2;
mod day3;

use std::env::args;
use self::day1::*;
use self::day2::*;
use day3::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();
    let day1_input_filename = args.get(1).expect("day1 input as first parameter");
    let day2_input_filename=  args.get(2).expect("day2 input as second parameter");
    let day3_input_filename=  args.get(3).expect("day3 input as third parameter");

    println!("day1: Part1 = {}", fuel_quantity_for_modules_only(day1_input_filename)?);
    println!("day1: Part2 = {}", fuel_quantity_for_modules_and_fuel_mass(day1_input_filename)?);
    println!("day2: Part1 = {}", program_first_place_value_during_1202(day2_input_filename)?);
    println!("day2: Part2 = {}", brut_force_program(day2_input_filename).expect("solution"));
    println!("day3: Part1 = {}", nearest_intersection_from_origin(day3_input_filename).expect("solution"));
    println!("day3: Part1 = {}", smallest_intersection_distance_from_origin(day3_input_filename).expect("solution"));

    Ok(())
}
