use std::clone::Clone;
#[derive(Debug)]
struct YouCanCloneMe {
    name: String,
    age: u8
}

impl Clone for YouCanCloneMe {
    fn clone(&self) -> Self {
        return YouCanCloneMe{name: self.name.clone(), age: self.age};
    }
}

fn main() {
    let yaroo = YouCanCloneMe {
        name: String::from("Name"),
        age: 22
    };

    println!("cloned yaroo: {:#?}", yaroo.clone());
}
