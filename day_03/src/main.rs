use std::convert::TryFrom;
use std::fmt;
use std::fs::read_to_string;
use std::io;

use common::part::is_part_one;
use common::reexport::itertools::Itertools;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

fn run() -> io::Result<()> {
    let input: Vec<String> = read_to_string("./input.txt")?
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    let wire1 = Wire::from_input(input.get(0).unwrap());
    let wire2 = Wire::from_input(input.get(1).unwrap());
    let intersects: Vec<Point> = wire1.intersects(&wire2);

    if is_part_one() {
        let min_distance = intersects
            .iter()
            .map(|p| (p.x.abs() + p.y.abs()) as u32) // convert to u32 to get min() function
            .min()
            .unwrap();

        println!("Result = {}", min_distance);
    } else {
        let min_steps = intersects
            .iter()
            .map(|p| {
                let wire1_steps = wire1.steps_to_point(*p);
                let wire2_steps = wire2.steps_to_point(*p);
                (wire1_steps + wire2_steps) as u32	// convert to u32 to get min() function
            })
            .min()
            .unwrap();

        println!("Result = {}", min_steps);
    }

    Ok(())
}

struct Wire {
    lines: Vec<LineSegment>,
}

impl Wire {
    fn from_input(input: &str) -> Wire {
        let mut last_point: Point = Default::default();
        let lines = input
            .split(',')
            .map(|s| {
                let dir = Direction::try_from(s).unwrap();
                let p1 = last_point;
                let p2 = p1.add_direction(&dir);
                last_point = p2;
                LineSegment::new(p1, p2)
            })
            .collect();
        Wire { lines }
    }

    // returns the points where the 2 wires intersect
    fn intersects(&self, other: &Wire) -> Vec<Point> {
        self.lines
            .iter()
            .cartesian_product(other.lines.iter())
            .filter_map(|(l1, l2)| l1.intersect(l2))
            .collect()
    }

    // returns the steps"length" from the start of the wire
    // till the point
    // Panics when the point is not on the Wire
    fn steps_to_point(&self, p: Point) -> f32 {
        // check if point p is on one line of wire
        if self.lines.iter().find(|l| l.has_point(p)).is_none() {
            panic!("Wire dont has point p: {}", p);
        }

        let mut steps = 0.;
        for line in self.lines.iter() {
            if line.has_point(p) {
                let line = line.set_b(p);
                steps += line.length();
                break;
            }
            steps += line.length();
        }
        steps
    }
}

enum Direction {
    Up(f32),
    Down(f32),
    Right(f32),
    Left(f32),
}

impl TryFrom<&str> for Direction {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let letter = s.chars().next().unwrap();
        let steps: f32 = s
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<f32>()
            .unwrap();

        match letter {
            'U' => Ok(Direction::Up(steps)),
            'D' => Ok(Direction::Down(steps)),
            'R' => Ok(Direction::Right(steps)),
            'L' => Ok(Direction::Left(steps)),
            _ => Err(format!("Error: Can't convert {} to Direction", s)),
        }
    }
}

#[derive(Default, Copy, Clone, PartialEq, Debug)]
struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    // Creates a new point horizontal/vertical
    // where the point of origin is self
    fn add_direction(self, dir: &Direction) -> Point {
        match dir {
            Direction::Up(z) => Point::new(self.x, self.y + z),
            Direction::Down(z) => Point::new(self.x, self.y - z),
            Direction::Right(z) => Point::new(self.x + z, self.y),
            Direction::Left(z) => Point::new(self.x - z, self.y),
        }
    }

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn scale(self, s: f32) -> Point {
        Point {
            x: self.x * s,
            y: self.y * s,
        }
    }

    fn cross(self, other: Point) -> f32 {
        self.x * other.y - self.y * other.x
    }

    // Distance between two points
    fn distance(self, other: Point) -> f32 {
        let sba = other.sub(self);
        (sba.x.powi(2) + sba.y.powi(2)).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(PartialEq)]
struct LineSegment {
    pub a: Point,
    pub b: Point,
}

impl LineSegment {
    fn new(a: Point, b: Point) -> Self {
        LineSegment { a, b }
    }

    // returns the point of intersection
    // and None when the line segments never cross
    fn intersect(&self, other: &LineSegment) -> Option<Point> {
        let r = self.b.sub(self.a);
        let s = other.b.sub(other.a);
        let rxs = r.cross(s);
        let qpxr = other.a.sub(self.a).cross(r);

        if rxs == 0. && qpxr != 0. {
            return None;
        }

        let t = other.a.sub(self.a).cross(s) / rxs;
        let u = other.a.sub(self.a).cross(r) / rxs;

        if rxs != 0. && (0. <= t && t <= 1.) && (0. <= u && u <= 1.) {
            return Some(r.scale(t).add(self.a));
        }
        None
    }

    // check if the point is on the line segment
    fn has_point(&self, p: Point) -> bool {
        let sba = self.b.sub(self.a);
        let spa = p.sub(self.a);
        let cross_product = spa.y * sba.x - spa.x * sba.y;

        if cross_product != 0. {
            return false;
        }

        let dot_product = spa.x * sba.x + spa.y * sba.y;
        if dot_product < 0. {
            return false;
        }

        let sqrd_len_ba = sba.x * sba.x + sba.y * sba.y;
        dot_product < sqrd_len_ba
    }

    // Length of the line segment
    fn length(&self) -> f32 {
        self.a.distance(self.b)
    }

    // set Point b to p
    fn set_b(&self, p: Point) -> LineSegment {
        LineSegment::new(self.a, p)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(5., 3.);
        let p2 = Point::new(5., 5.);
        let p3 = Point::new(-2., 5.);

        assert_eq!(p1.distance(p2), 2.);
        assert_eq!(p2.distance(p3), 7.);
    }

    #[test]
    fn test_line_intersect() {
        let line1 = LineSegment::new(Point::new(1., 2.), Point::new(2., 4.));
        let line2 = LineSegment::new(Point::new(2., 1.), Point::new(4., 3.));
        let line3 = LineSegment::new(Point::new(2., 3.), Point::new(4., 1.));

        assert_eq!(line1.intersect(&line2), None);
        assert_eq!(line2.intersect(&line3), Some(Point::new(3., 2.)));
    }

    #[test]
    fn test_line_has_point() {
        let line = LineSegment::new(Point::new(3., 2.), Point::new(3., 5.));
        let c = Point::new(3., 4.);
        let d = Point::new(4., 4.);

        assert_eq!(line.has_point(c), true);
        assert_eq!(line.has_point(d), false);
    }
}
