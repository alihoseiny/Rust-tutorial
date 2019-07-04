#[cfg(test)]
mod test10 {

    #[derive(Debug)]
    struct Student {
        name: String,
        id: u32,
        graduated: bool
    }

    fn create_student(name: String, id: u32, graduated: bool) -> Student {
        Student {
            name,
            id,
            graduated
        }
    }

    #[test]
    fn test_struct_construction() {
        let asghar = Student {
            id: 963258741,
            name: String::from("اصغر اکبرآبادی اصل"),
            graduated: false
        };
    }

    #[test]
    fn test_struct_printing() {
        let asghar = Student {
            id: 963258741,
            name: String::from("اصغر اکبرآبادی اصل"),
            graduated: false
        };
        println!("{:?}", asghar);
    }

    #[test]
    fn test_struct_value_access() {
        let asghar = Student {
            id: 963258741,
            name: String::from("اصغر اکبرآبادی اصل"),
            graduated: false
        };
        assert_eq!("اصغر اکبرآبادی اصل", asghar.name);
    }

    #[test]
    fn test_value_modification() {
        let mut asghar = Student {
            id: 963258741,
            name: String::from("اصغر اکبرآبادی اصل"),
            graduated: false
        };
        assert_eq!(asghar.name, "اصغر اکبرآبادی اصل");
        asghar.name = String::from("اکبر اصغر زاده");
        assert_eq!(asghar.name, "اکبر اصغر زاده");
    }

    #[test]
    fn test_struct_construction_using_factory_func() {
        let asghar = create_student(String::from("اصغر اکبرآبادی اصل"), 963258741, true);
        assert_eq!(asghar.name, "اصغر اکبرآبادی اصل");
    }
}