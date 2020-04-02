#[cfg(test)]
mod test21 {
    #[test]
    #[should_panic]
    fn test_panic_vector_invalid_index() {
        let a = vec![1, 2, 3, 4, 5, 6];
        println!("This program will panic and this line never will print. {}", a[100]);
    }
}