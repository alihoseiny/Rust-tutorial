#[cfg(test)]
mod test1 {
    #[test]
    fn test_variable_declaration(){
        let var1;
        let var2 = 10;
        let var3: i32 = 20;
        var1 = 50;
    }

    #[test]
    fn test_reassigning_mutable_var() {
        let mut x = 10;
        x = 5;
    }


}