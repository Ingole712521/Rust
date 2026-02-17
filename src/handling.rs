pub fn handling() {
    fn pratice() {
        fn divide(a: i32, b: i32) -> Result<i32, String> {
            if b == 0 {
                Err(String::from("Cannot diived by zero"))
            } else {
                Ok(a / b)
            }
        }

        match divide(100, 20) {
            Ok(value) => println!("the result is {}", value),
            Err(err) => println!("{}", err),
        }
    }
    pratice();
}
