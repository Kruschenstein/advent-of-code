use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::iter;

fn fuel_quantity_for_mass(mass: i32) -> i32 {
    mass / 3 - 2
}

fn modules_mass(filename: &str) -> io::Result<Vec<i32>> {
    let file = File::open(filename)?;
    let buffer = BufReader::new(file);

    Ok(buffer.lines().into_iter()
        .map(|line| line.unwrap())
        .map(|line| line.parse().expect("number"))
        .collect())
}

fn fuel_quantity_for_modules(modules_mass: &Vec<i32>) -> Vec<i32> {
    modules_mass.iter().map(|i| fuel_quantity_for_mass(*i)).collect()
}

pub fn fuel_quantity_for_modules_only(filename: &str) -> io::Result<i32> {
    Ok(fuel_quantity_for_modules(&modules_mass(filename)?).iter().sum())
}

fn fuel_quantity_for_fuel_quantity(fuel_quantity: i32) -> i32 {
    let fuel_quantity_needed_for_fuel_quantity = |acc: &mut i32, _| {
        let result = Some(*acc);
        *acc = fuel_quantity_for_mass(*acc);
        result
    };
    iter::repeat(())
        .scan(fuel_quantity, fuel_quantity_needed_for_fuel_quantity)
        .take_while(|i| *i > 0)
        .sum::<i32>()
}

pub fn fuel_quantity_for_modules_and_fuel_mass(filename: &str) -> io::Result<i32> {
    Ok(fuel_quantity_for_modules(&modules_mass(filename)?)
        .iter().map(|i| fuel_quantity_for_fuel_quantity(*i)).sum())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fuel_quantity_for_modules_should_apply_fuel_formula_to_each_module() {
        assert_eq!(vec![2, 2, 654, 33583],
                   fuel_quantity_for_modules(&vec![12, 14, 1969, 100756]));
    }

    #[test]
    fn fuel_quantity_for_fuel_quantity_should_sum_the_fuel_needed_for_the_fuel() {
        assert_eq!(16, fuel_quantity_for_fuel_quantity(14));
        assert_eq!(2935, fuel_quantity_for_fuel_quantity(1969));
        assert_eq!(151102, fuel_quantity_for_fuel_quantity(100756));
    }
}