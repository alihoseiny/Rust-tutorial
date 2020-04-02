use std::error;
use std::fmt;

#[derive(Debug, Clone)]
struct MyError;

impl error::Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oooops.")
    }
}


fn main() {
    let func_result = get_always_error();
    match func_result {
        Err(error) =>     println!("Error message: {}", error),
        _ => {}
    }
}

fn get_always_error() -> Result<i8, MyError> {
    let my_error: MyError = MyError{};
    return Err(my_error);
}
