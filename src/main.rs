use std::env::args;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn modules_mass(filename: &str) -> io::Result<impl Iterator<Item = i32>> {
    let file = File::open(filename)?;
    let buffer = BufReader::new(file);

    Ok(buffer.lines().into_iter()
        .map(|line| line.unwrap())
        .map(|line| line.parse().expect("number")))
}

fn fuel_quantity_for_mass(mass: i32) -> i32 {
    mass / 3 - 2
}

fn fuel_quantity_for_modules(filename: &str) -> io::Result<impl Iterator<Item = i32>> {
    Ok(modules_mass(filename)?
        .map(fuel_quantity_for_mass))
}

fn fuel_quantity_for_fuel_quantity(fuel_quantity: i32) -> i32 {
    let mut res = vec![];
    let mut actual_fuel_quantity = fuel_quantity;
    while actual_fuel_quantity > 0 {
        res.push(actual_fuel_quantity);
        actual_fuel_quantity = fuel_quantity_for_mass(actual_fuel_quantity);
    }
    res.iter().sum()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();
    let filename = args.get(1).expect("at least filename parameter");

    println!("day1: Part1 = {}", fuel_quantity_for_modules(filename)?.sum::<i32>());
    println!("day1: Part2 = {}", fuel_quantity_for_modules(filename)?
        .map(fuel_quantity_for_fuel_quantity).sum::<i32>());

    Ok(())
}
