trait Fly {
    fn fly(&self);
    fn land(&self) {
        println!("Flyable Object now landing.");
    }
}

impl Fly for String {
    fn fly(&self) {
        println!("Oh my گاج. It's a flying string!");
    }
}

fn main() {
        let flying_string = String::from("بغبغو");
        flying_string.fly();
}
