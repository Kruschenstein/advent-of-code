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
        let mut points = vec![];
        let mut x = 0;
        let mut y = 0;
        for direction in wire_direction {
            let new_origin = direction.new_position(&(x, y));
            points.push(((x, y), new_origin));
            x = new_origin.0;
            y = new_origin.1;
        }
        res.push(points);
    }

    fn is_between(x_min: &i32, x: &i32, x_max: &i32) -> bool {
        x_min <= x && x <= x_max
    }

    let mut result = vec![];
    for (i, e1) in res.iter().enumerate() {
        for e2 in &res[i + 1..] {
            for s in e1 {
                for t in e2 {
                    match (s, t) {
                        (((x1, y1), (x2, y2)),
                            ((x3, y3), (x4, y4))) |
                        (((x3, y3), (x4, y4)),
                            ((x1, y1), (x2, y2)))
                        if x1 == x2 && y3 == y4 &&
                            (is_between(x3, x1, x4) || is_between(x4, x1, x3)) &&
                            (is_between(y1, y3, y2) || is_between(y2, y3, y1))
                        =>
                            result.push((x1, y3)),
                        _ => ()
                    }
                }
            }
        }
    }

    Ok(result.iter().filter(|(x, y)| *x + *y != 0).map(|(x, y)| x.abs() + y.abs()).min().expect("result"))
}