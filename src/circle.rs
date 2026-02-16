use std::io;
pub fn circle() {
    // fn value() {
    //     let mut num = 1;
    //     while num < 5 {
    //         println!("{}", num);
    //         num += 1;
    //     }
    // }

    // fn even() {
    //     let mut num = String::new();
    //     io::stdin().read_line(&mut num).unwrap();
    //     let num: i32 = num.trim().parse().unwrap();

    //     for num in 1..num {
    //         if num % 2 == 0 {
    //             print!("{}  ", num)
    //         }
    //     }
    // }

    // fn breakh() {
    //     let mut num = 1;
    //     loop {
    //         if num == 5 {
    //             break;
    //         }
    //         println!("{}", num);
    //         num += 1;
    //     }
    // }

    // fn chala() {
    //     let mut num = String::new();
    //     io::stdin().read_line(&mut num).unwrap();

    //     let nums = num.trim().parse().unwrap();

    //     for nums in 1..=nums {
    //         if nums == 6 {
    //             continue;
    //         }
    //         println!("{}", nums)
    //     }
    // }

    fn factorial() {
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        let number: i32 = num.trim().parse().unwrap();
        let mut fact = 1;

        for number in 1..=number {
            fact *= number;
        }
        println!("{}", fact)
    }

    factorial();

    // chala();
    // breakh();
    // value();
    // even();
}
