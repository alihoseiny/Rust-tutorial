fn show_store_status(hour: f32) {
    match hour {
        8.0...12.0 | 13.0...18.0 => println!("The store is open."),
        12.0...13.0 => println!("We will start working again at 13"),
        _ => println!("closed")
                                            }
}

fn main() {
    let hours = [8.5, 12.0, 12.25, 13.0, 19.0];
    for hour in hours.iter() {
        show_store_status(*hour);
    }
}
