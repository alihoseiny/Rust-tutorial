#[derive(Debug)]
struct CMYK (u8, u8, u8, u8);

#[derive(Debug)]
struct IPv4 (u8, u8, u8, u8);

fn main() {
    let red = CMYK(0, 1, 1, 0);
    let local_ip = IPv4(127, 0, 0, 1);
    println!("red color {:?} and local ip {:?}. These values have different types.", red, local_ip);
}

