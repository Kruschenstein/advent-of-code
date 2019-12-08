mod day1;

use std::env::args;
use self::day1::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();
    let filename = args.get(1).expect("at least filename parameter");

    println!("day1: Part1 = {}", fuel_quantity_for_modules_only(filename)?);
    println!("day1: Part2 = {}", fuel_quantity_for_modules_and_fuel_mass(filename)?);

    Ok(())
}
