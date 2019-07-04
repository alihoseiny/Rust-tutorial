#[derive(Debug)]
struct UnitLikeStruct;

fn main() {
    let my_unit = UnitLikeStruct;
    let same_unit_as_my_unit = UnitLikeStruct {};
    println!("my_unit: {:?}, same_unit_as_my_unit: {:?}", my_unit, same_unit_as_my_unit);
}

