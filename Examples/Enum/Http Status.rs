enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500
}

fn main() {
    let a = HttoStatus::Ok;
    let b = HttoStatus::NotFound;
    let c = HttoStatus::InternalServerError;
    println!("Ok value in the memory: {}", a as u8);
    println!("NotFound value in the memory: {}", b as u16);
    println!("InternalServerError value in the memory: {}", c as u16);
}
