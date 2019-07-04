#[cfg(test)]
mod test4 {
    #[test]
    fn test_if_else() {
        let a = 5;
        if a < 10{
        }
        else {
        panic!("Should not run the else");
        }
    }

    #[test]
    fn test_multiple_else_if() {
        let user_type = "Guest";
        let mut chosen_message: &str;
        if user_type == "Admin"{

            chosen_message = "Hello dear Admin. You have full access to all the settings and you can change \
                      them if you want.";
        }
        else if user_type == "Member"{
            chosen_message = "Welcome our beautiful member!";
       }
        else if user_type == "Guest" {
            chosen_message = "Please register dear guest.";
        }
        else{
            chosen_message = "I can not understand who you are. Get out right now.";
        }

        assert_eq!(chosen_message, "Please register dear guest.")
    }

    #[test]
    fn test_ternary_assignment() {
        let condition = true;
        let a = if condition {
            0
        }
        else{
            1
        };
        assert_eq!(a, 0);
    }
}