#[derive(Debug)]
struct TestStruct {
        val1: i32,
        val2: u8,
        val3: i64,
        val4: f64,
        val5: u16
}

fn main() {
    let struct1 = TestStruct {
        val1: -1238,
        val2: 5,
        val3: -6464564564,
        val4: 1234.5678,
        val5: 15
    };
    let struct2 = TestStruct {val5: 236, ..struct1};
    println!("struct1: {:#?}", struct1);
    println!("struct2: {:#?}", struct2);
}

