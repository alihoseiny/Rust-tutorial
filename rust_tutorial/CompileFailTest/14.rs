///
///```compile_fail
/// fn print_message(user_level: u8) {
///    match user_level {
///        0 => println!("Welcome dear admin."),
///        1 => println!("Welcome back our best member."),
///        2 => println!("Hello. Please register first.")
///    }
///}
///
///fn main() {
///    let user_inputs: [u8;5] = [0, 1, 2, 3, 4];
///    for value in user_inputs.iter() {
///        print_message(*value);
///    }
///}
/// ```
fn test1(){}

///
/// ```compile_fail
/// struct RgbStruct {
///    red: u16,
///    green: u16,
///    blue: u16
///}
///
///fn print_number(n: u16) {
///    println!("number is: {}", n);
///}
///
///fn main() {
///    let colours = [
///        RgbStruct {
///            red: 112,
///            green: 0,
///            blue: 0
///        },
///        RgbStruct {
///            red: 123,
///            green: 124,
///            blue: 8
///        },
///        RgbStruct {
///            red: 0,
///            green: 41,
///            blue: 223
///        }
///    ];
///
///    for rgb_reference in colours.iter() {
///        match rgb_reference {
///            RgbStruct {red, blue: 0, green: 0} => {
///                println!("This is a kind of red colour.");
///                print_number(red);
///            },
///            RgbStruct {red, green, blue} => println!("rgb({}, {}, {})", red, green, blue)
///        }
///    }
///}
/// ```
fn test2(){}


///
/// ```compile_fail
/// struct RgbStruct {
///    red: u16,
///    green: u16,
///    blue: u16
///}
/// fn extract_and_match(colour: RgbStruct) {
///    match colour {
///        new_colour @ RgbStruct {..} => println!("This is my colour: rgb({}, {}, {})", new_colour.red, new_colour.green, new_colour.blue),
///    }
///}
///
/// let colour = RgbStruct {
///        red: 120,
///        green: 0,
///        blue: 255
///    };
///
/// match colour {
///        new_colour @ RgbStruct {..} => println!("This is my colour: rgb({}, {}, {})", new_colour.red, new_colour.green, new_colour.blue),
///    }
///
/// println!("{:?}", colour);
/// ```
///
fn test3(){}