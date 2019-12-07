use std::env::args;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn modules_mass(filename: &str) -> io::Result<impl Iterator<Item = u32>> {
    let file = File::open(filename)?;
    let buffer = BufReader::new(file);

    Ok(buffer.lines().into_iter()
        .map(|line| line.unwrap())
        .map(|line| line.parse().expect("number")))
}

fn fuel_quantity_for_mass(mass: u32) -> u32 {
    mass / 3 - 2
}

fn fuel_quantity_for_modules(filename: &str) -> io::Result<u32> {
    Ok(modules_mass(filename)?
        .map(fuel_quantity_for_mass)
        .sum())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();
    let filename = args.get(1).expect("at least filename parameter");
    let fuel_quantity_for_modules = fuel_quantity_for_modules(filename)?;
    println!("day1: res = {}", fuel_quantity_for_modules);

    Ok(())
}
