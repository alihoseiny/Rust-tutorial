use std::ops::Mul;

struct Point {
    x: u8,
    y: u8
}


impl Mul<u8> for Point {
    type Output = Point;

    fn mul(self, rhs: u8) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

fn main() {
    let my_point = Point{
        x: 10,
        y: 11
    };
    let new_point = my_point * 10;
    println!("new point. x: {}, y: {}", new_point.x, new_point.y);
}


