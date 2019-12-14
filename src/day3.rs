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

struct Segment(Point, Point);

impl Segment {
    fn distance(&self) -> i32 {
        ((((self.0).0 - (self.1).0).pow(2)
            + ((self.0).1 - (self.1).1).pow(2)) as f64).sqrt() as i32
    }
}

struct WireInfo {
    segment: Segment,
    distance_from_origin: i32,
}

struct IntersectionInfo {
    intersection: Point,
    distance_from_origin: i32,
}

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

    fn distance_from(&self, distance: i32) -> i32 {
        distance + *match self {
            Direction::Up(val) => val,
            Direction::Right(val) => val,
            Direction::Down(val) => val,
            Direction::Left(val) => val,
        }
    }
}

fn compute_intersection_info(filename: &str) -> GenResult<Vec<IntersectionInfo>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let wires_direction: Vec<Vec<Direction>> = reader.lines().into_iter()
        .map(|line| line.expect("enumeration of direction")
            .split(',')
            .map(|direction| Direction::from(direction).expect("valid direction"))
            .collect())
        .collect();

    Ok(intersections(&wires_segments(wires_direction)))
}

pub fn nearest_intersection_from_origin(filename: &str) -> GenResult<i32> {
    compute_intersection_info(filename).map(|intersection| intersection.iter()
        .map(|intersection| intersection.intersection)
        .filter(|(x, y)| *x + *y != 0)
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .expect("result"))
}

pub fn smallest_intersection_distance_from_origin(filename: &str) -> GenResult<i32> {
    compute_intersection_info(filename).map(|intersection| intersection.iter()
        .map(|intersection| intersection.distance_from_origin)
        .filter(|distance | *distance != 0)
        .min()
        .expect("result"))
}

fn wires_segments(wires_direction: Vec<Vec<Direction>>) -> Vec<Vec<WireInfo>> {
    let mut res = vec![];
    for wire_direction in wires_direction {
        let mut points = vec![];
        let mut origin = (0, 0);
        let mut distance = 0;
        for direction in wire_direction {
            let new_origin = direction.new_position(&origin);
            points.push(WireInfo { distance_from_origin: distance, segment: Segment(origin, new_origin) });
            origin = new_origin;
            distance = direction.distance_from(distance);
        }
        res.push(points);
    }
    res
}

fn intersections(res: &Vec<Vec<WireInfo>>) -> Vec<IntersectionInfo> {
    let mut result = vec![];
    for (i, e1) in res.iter().enumerate() {
        for e2 in &res[i + 1..] {
            result.append(&mut find_intersection_points(e1, e2));
        }
    }
    result
}

fn find_intersection_points(e1: &Vec<WireInfo>, e2: &Vec<WireInfo>) -> Vec<IntersectionInfo> {
    let mut res = vec![];
    for s in e1 {
        for t in e2 {
            match &(&s.segment, &t.segment) {
                (Segment((x1, y1), (x2, y2)),
                    Segment((x3, y3), (x4, y4))) |
                (Segment((x3, y3), (x4, y4)),
                    Segment((x1, y1), (x2, y2)))
                if x1 == x2 && y3 == y4 &&
                    is_framed(x3, x1, x4) && is_framed(y1, y3, y2) => {
                    let Segment(s_first, _) = s.segment;
                    let Segment(t_first, _) = t.segment;
                    let intersection = (*x1, *y3);
                    let distance_from_origin= s.distance_from_origin + t.distance_from_origin +
                        Segment(s_first, intersection).distance() +
                        Segment(t_first, intersection).distance();

                    res.push(IntersectionInfo { intersection, distance_from_origin })
                }
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
