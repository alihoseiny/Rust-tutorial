#[derive(Debug)]
struct  MyStruct<'a> {
    name: &'a str,
    age: u8
}

impl<'a> MyStruct<'a> {
    fn change_name(&mut self, new_name: &'a str) -> &str {
        let old_name = self.name;
        self.name = new_name;
        return old_name;
    }
}

fn main() {
    let name = "Asghar";
    let mut person = MyStruct {
        name,
        age: 10
    };
    
    println!("{:?}", person);

    person.change_name("Akbar");

    println!("{:?}", person);
}
