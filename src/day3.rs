use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

type GenError = Box<dyn std::error::Error>;
type GenResult<T> = Result<T, GenError>;

enum Direction {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

type Point = (i32, i32);

impl Direction {
    fn from(string: &str) -> GenResult<Direction> {
        let num = &string[1..];
        match string.chars().next() {
            Some('U') => Ok(Direction::Up(num.parse()?)),
            Some('R') => Ok(Direction::Right(num.parse()?)),
            Some('D') => Ok(Direction::Down(num.parse()?)),
            Some('L') => Ok(Direction::Left(num.parse()?)),
            _ => Err(Box::from(Error::new(ErrorKind::Other, "not existing instruction"))),
        }
    }

    fn draw_line_from(&self, (x, y): &Point) -> HashSet<Point> {
        match self {
            Direction::Up(val) => (0..*val).map(|i| (*x, *y + i)).collect(),
            Direction::Right(val) => (0..*val).map(|i| (*x + i, *y)).collect(),
            Direction::Down(val) => (0..*val).map(|i| (*x, *y - i)).collect(),
            Direction::Left(val) => (0..*val).map(|i| (*x - i, *y)).collect(),
        }
    }

    fn new_position(&self, (x, y): &Point) -> Point {
        match self {
            Direction::Up(val) => (*x, *y + *val),
            Direction::Right(val) => (*x + *val, *y),
            Direction::Down(val) => (*x, *y - *val),
            Direction::Left(val) => (*x - *val, *y),
        }
    }
}

pub fn smallest_intersection(filename: &str) -> GenResult<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let wires_direction: Vec<Vec<Direction>> = reader.lines().into_iter()
        .map(|line| line.expect("enumeration of direction")
            .split(',')
            .map(|direction| Direction::from(direction).expect("valid direction"))
            .collect())
        .collect();

    let mut res = vec![];
    for wire_direction in wires_direction {
        let mut points = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        for direction in wire_direction {
            points = &points | &direction.draw_line_from(&(x, y));
            let new_origin = direction.new_position(&(x, y));
            x = new_origin.0;
            y = new_origin.1;
        }
        res.push(points);
    }

    let result = &res[0] & &res[1];

    Ok(result.iter().filter(|(x, y)| *x + *y != 0).map(|(x, y)| x.abs() + y.abs()).min().expect("result"))
}