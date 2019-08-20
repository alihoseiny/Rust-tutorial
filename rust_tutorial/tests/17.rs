#[cfg(test)]
mod test17 {
    trait Fly {
        fn new() -> Self;
        fn fly(&self);
        fn land(&self) {
            println!("Flyable Object now landing.");
        }
    }

    struct Kaftar ();

    impl Fly for Kaftar {
        fn new() -> Self {
            return Kaftar{};
        }

        fn fly(&self) {
            println!("Kafter The Kakol Be Sar is flying");
        }
    }

    trait Brush {
        fn draw(&self);
        fn change_colour(&self);
    }

    trait Screen {
        fn draw(&self);
        fn turnoff(&self);
    }

    struct Something();

    impl Brush for Something {
        fn draw(&self) {
            println!("Draw a line on the paper.");
        }

        fn change_colour(&self) {
            println!("Brush colour changed.");
        }
    }

    impl Screen for Something {
        fn draw(&self) {
            println!("Draw a line on the screen");
        }

        fn turnoff(&self) {
            println!("Screen turned off");
        }
    }


    #[test]
    fn test_associated_function_call_syntax() {
        let kakol_be_sar = Kaftar::new();
        kakol_be_sar.fly();
    }

    #[test]
    fn test_calling_method_by_trait() {
        let something = Something{};
        Brush::draw(&something);
        Screen::draw(&something);
    }

    #[test]
    fn test_calling_method_by_type() {
        let x = -10;
        i32::abs(x);
    }
}