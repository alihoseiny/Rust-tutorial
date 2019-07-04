#[cfg(test)]
mod test12 {
    #[derive(Debug)]
    struct TestStruct {
        val1: i32,
        val2: u8,
        val3: i64,
        val4: f64,
        val5: u16
    }

    #[derive(Debug, Clone)]
    struct Student {
        name: String,
        id: u32,
        courses: [Course; 3]
    }

    #[derive(Debug, Clone)]
    struct Course {
        name: String,
        passed: bool
    }

    #[derive(Debug)]
    struct TupleLike (u8, u8, u8);

    #[derive(Debug)]
    struct UnitLikeStruct;

    #[test]
    fn test_struct_inheriting_similar_values() {
        let struct1 = TestStruct {
            val1: -1238,
            val2: 5,
            val3: -6464564564,
            val4: 1234.5678,
            val5: 15
        };
        let struct2 = TestStruct {val5: 236, ..struct1};
        assert_eq!(struct1.val1, struct2.val1);
        assert_eq!(struct1.val2, struct2.val2);
        assert_eq!(struct1.val3, struct2.val3);
        assert_eq!(struct1.val4, struct2.val4);
        assert_ne!(struct1.val5, struct2.val5);
        assert_eq!(struct2.val5, 236);
    }

    #[test]
    ///
    /// Only test syntax. not functionality
    fn test_inheritance_with_clone() {
        let courses = [Course {name: String::from("درس۱"), passed: false},
            Course {name: String::from("درس۲"), passed: false},
            Course {name: String::from("درس۳"), passed: false}];

        let student1 = Student {
            name: String::from("اصغر اکبرزاده اصل"),
            id: 97959493,
            courses
        };

        let student2 = Student {id: 98999694, ..student1.clone()};
    }

    #[test]
    fn test_tuple_like_struct() {
        let mut tuple_like = TupleLike(10, 11, 13);
        assert_eq!(tuple_like.0, 10);
        tuple_like.0 = 18;
        assert_eq!(tuple_like.0, 18);
    }

    #[test]
    ///
    /// Only test syntax.
    fn test_unit_like_struct() {
        let my_unit = UnitLikeStruct;
        let same_unit_as_my_unit = UnitLikeStruct {};
    }

    

}