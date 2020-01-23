#[derive(Debug)]
enum Something <'a> {
    Some(&'a u8),
    Thing(u8)
}

fn main() {
    let number: u8 = 10;
    let a = Something::Some(&number);
    println!("{:?}", a);
}
