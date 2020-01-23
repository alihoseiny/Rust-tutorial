fn main() {
    call_me();
    call_me();
    call_me();
    call_me();
}

fn call_me() {
    static mut NUMBER_OF_CALLS: u8 = 0;
    unsafe {
        NUMBER_OF_CALLS += 1;
        println!("number of calls: {}", NUMBER_OF_CALLS);
    }
}
