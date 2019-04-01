struct Foo(u8, u8);

fn print_guarded_pattern(input: Foo) {
    match input {
        Foo(x, _) if x % 2 == 0 => println!("{} is an even number", x),
        Foo(x, y) => println!("first value is not even. pair is: ({}, {})", x, y)
    }
}

fn main() {
    print_guarded_pattern(Foo(2, 10));
    print_guarded_pattern(Foo(7, 8));
}
