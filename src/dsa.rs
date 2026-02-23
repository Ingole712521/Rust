use std::io;

pub fn dsa() {
    // fn print_num() {
    //     let mut num = String::new();
    //     io::stdin().read_line(&mut num).unwrap();

    //     let number = num.trim().parse().unwrap();

    //     for number in 1..=number {
    //         println!("{}", number)
    //     }
    // }

    // fn even() {
    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).unwrap();
    //     let number = input.trim().parse().unwrap();
    //     for i in 1..=number {
    //         if i % 2 == 0 {
    //             println!("{} it is a even number", i);

    //         }else {
    //             println!("{} not a even number" , i)
    //         }
    //     }
    // }

    // fn sum() {
    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).unwrap();

    //     let number: i32 = input.trim().parse().expect("Invalid Number");

    //    let mut total = 0;
    //    for i in 1..=number{
    //     total += i;
    //    }
    //    println!("{}", total);
    // }

    // fn fact() {
    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).unwrap();
    //     let number = input.trim().parse().expect("Invalid number");

    //     if number < 0 {
    //         println!("Input number ");
    //         return;
    //     }

    //     let mut total = 1;
    //     for i in 1..=number {
    //         total *= i;
    //     }
    //     println!("{}", total);
    // }

    // String Method

    // fn length() {
    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).unwrap();
    //     let number = input.trim().len();

    //     println!("the length of the number is {}", number)
    // }

    // Math Method which is really use mostly

    // fn length() {
    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).unwrap();
    //     let mut number: i32 = input.trim().parse().unwrap();

    //     let mut count = 0;

    //     while number != 0 {
    //         number /= 10;
    //         count += 1;
    //     }
    //     println!("COunt is {count}")
    // }

    // Reverse String

    fn reverse() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let number: i32 = input.trim().parse().expect("Invalid Message");

        let mut num = number.abs();
        let mut reversed = 0;

        while num > 0 {
            reversed = reversed * 10 + num % 10;
            num /= 10;
        }

        let result = if number < 0 { 1 } else { reversed };

        println!("result is {}", result)
    }

    reverse();

    // length();
    // fact();
    // sum();
    // even();
    // print_num();
}
