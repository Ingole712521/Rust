pub fn handling() {
    // fn pratice() {
    //     fn divide(a: i32, b: i32) -> Result<i32, String> {
    //         if b == 0 {
    //             Err(String::from("Cannot diived by zero"))
    //         } else {
    //             Ok(a / b)
    //         }
    //     }

    //     match divide(100, 20) {
    //         Ok(value) => println!("the result is {}", value),
    //         Err(err) => println!("{}", err),
    //     }
    // }

    // fn pratice() {
    //     fn check_age(age: i32) -> Result<String, String> {
    //         if age == 0 {
    //             Err(String::from("Not eligible"))
    //         } else if age >= 18 {
    //             Ok(String::from("eligible"))
    //         } else {
    //             Err(String::from("Not eligible"))
    //         }
    //     }

    //     match check_age(90) {
    //         Ok(value) => println!("{}", value),
    //         Err(err) => println!("{}", err),
    //     }
    // }

    fn pratice() {
        fn check_age(age: i32) -> Result<String, String> {
            if age > 18 {
                Ok(String::from("Eligible"))
            } else {
                Err(String::from("Not Eligible"))
            }
        }

        match check_age(20) {
            Ok(value) => println!("{}", value),
            Err(err) => println!("{}", err),
        }
    }

    pratice();
}
