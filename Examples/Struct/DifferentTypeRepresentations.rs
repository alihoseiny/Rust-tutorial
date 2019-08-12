/*
 * For running performance-test.py file, you need compile this file with C
 * Representation with following command:
 * rustc DifferentTypeRepresentations.rs -o clike.o
 * The you should compile this file with Rust representation with following command:
 * rustc DifferenetTypeRepresentations.rs -o rlike.o
 * Then you can run that script for performance testing
 * */
#[repr(C)]  // Comment This Line if You Want to use Rust Representation
#[derive(Debug)]
struct CLikeStruct {
    p1: i32,
    p2: i8,
    p3: i32,
    p4: i8,
    p5: f64
}

fn main() {
    for i in 0..10000 {
        CLikeStruct{ p1: i - 10, p2: 10, p3: i * 11, p4: 20, p5: 3000.3000 };
    }
}
