enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500
}

fn print_status(http_status: HttpStatus) {
    match http_status {
        HttpStatus::Ok => println!("status: 200 Ok"),
        HttpStatus::NotFound => println!("status: 404 Not Found"),
        HttpStatus::InternalServerError => println!("status: 500 Internal Server Error")
    }
}

fn main() {
    let a = HttpStatus::Ok;
    let b = HttpStatus::NotFound;
    let c = HttpStatus::InternalServerError;
    print_status(a);
    print_status(b);
    print_status(c);
}
