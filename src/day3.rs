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
type Segment = ((i32, i32), (i32, i32));

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

    Ok(intersections(&wires_segments(wires_direction)).iter()
        .filter(|(x, y)| *x + *y != 0)
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .expect("result"))
}

fn wires_segments(wires_direction: Vec<Vec<Direction>>) -> Vec<Vec<Segment>> {
    let mut res = vec![];
    for wire_direction in wires_direction {
        let mut points = vec![];
        let mut origin = (0, 0);
        for direction in wire_direction {
            let new_origin = direction.new_position(&origin);
            points.push((origin, new_origin));
            origin = new_origin;
        }
        res.push(points);
    }
    res
}

fn intersections(res: &Vec<Vec<((i32, i32), (i32, i32))>>) -> Vec<(i32, i32)> {
    let mut result = vec![];
    for (i, e1) in res.iter().enumerate() {
        for e2 in &res[i + 1..] {
            result.append(&mut find_intersection_points(e1, e2));
        }
    }
    result
}

fn find_intersection_points(e1: &Vec<Segment>, e2: &Vec<Segment>) -> Vec<Point> {
    let mut res = vec![];
    for s in e1 {
        for t in e2 {
            match (s, t) {
                (((x1, y1), (x2, y2)),
                    ((x3, y3), (x4, y4))) |
                (((x3, y3), (x4, y4)),
                    ((x1, y1), (x2, y2)))
                if x1 == x2 && y3 == y4 &&
                    is_framed(x3, x1, x4) && is_framed(y1, y3, y2) =>
                    res.push((*x1, *y3)),
                _ => ()
            }
        }
    }
    res
}

fn is_between(x_min: &i32, x: &i32, x_max: &i32) -> bool {
    x_min <= x && x <= x_max
}

fn is_framed(x1: &i32, x: &i32, x2: &i32) -> bool {
    is_between(x1, x, x2) || is_between(x2, x, x1)
}
