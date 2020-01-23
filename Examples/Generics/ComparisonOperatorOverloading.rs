use std::cmp::Ordering;

struct Point {
    x: u8,
    y: u8
}


impl PartialEq<Point> for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}


impl PartialOrd<Point> for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x + self.y > other.x + other.y {
            return Some(Ordering::Greater);
        } else if self.x + self.y == other.x + other.y {
            return Some(Ordering::Equal);
        } else {
            return Some(Ordering::Less);
        }
    }
}

fn main() {
    let my_point = Point{
        x: 14,
        y: 11
    };
    let other_point = Point {
        x: 1,
        y: 13
    };
    println!("my_point == other_point is: {}", my_point > other_point);
}


