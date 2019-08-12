#[cfg(test)]
mod test16 {
    trait Fly {
        fn fly(&self);
        fn land(&self) {
            println!("Flyable Object now landing.");
        }
    }

    struct Kaftar ();
    struct AirPlane();
    struct UnknownFlyable();

    impl Fly for UnknownFlyable {
        fn fly(&self) {
            println!("Unknown flyable is flying.");
        }
    }

    impl Fly for Kaftar {
        fn fly(&self) {
            println!("Kafter The Kakol Be Sar is flying");
        }

        fn land(&self) {
            println!("Kafter The Kakol Be Sar is landing");
        }

    }

    impl Fly for AirPlane {
        fn fly(&self) {
            println!("Airplane is flying.");
        }

        fn land(&self) {
            println!("Airplane is landing.");
        }
    }

    impl Fly for String {
    fn fly(&self) {
        println!("Oh my گاج. It's a flying string!");
    }
}

    #[test]
    fn test_default_method_syntax() {
        let airplane = AirPlane{};
        let kakol_be_sar = Kaftar{};
        let unknown = UnknownFlyable{};

        airplane.land();
        kakol_be_sar.land();
        unknown.land();
    }

    #[test]
    fn test_implementing_custom_traits_for_default_types_syntax() {
        let flying_string = String::from("بغبغو");
        flying_string.fly();
    }
}