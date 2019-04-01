struct BigStruct {
    key1: u16,
    key2: u16,
    key3: u16,
    key4: u16,
    key5: u16,
    key6: u16,
    key7: u16,
}

fn main() {
    let a = BigStruct {
        key1: 0,
        key2: 1,
        key3: 2,
        key4: 3,
        key5: 4,
        key6: 5,
        key7: 6
    };

    match a {
        BigStruct {key1: 0, key7: x, ..} => println!(" x = {}", x),
        BigStruct {key6: y, ..} => println!(" y = {}", y)
    }
}
