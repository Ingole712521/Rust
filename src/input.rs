use std::io;

pub fn input() {
    // name and print
    // fn print(){
    //     let mut name = String::new();
    //     println!("Enter the value of name");
    //     io::stdin().read_line(&mut name).unwrap();

    //     let num: i32 = name.trim().parse().unwrap();
    //     println!("The value of num is {}" , num)
    // }

    // sum
    // fn sumdigit(){

    //     let mut a = String::new();
    //     let mut b = String::new();

    //     io::stdin().read_line(&mut a).unwrap();
    //     io::stdin().read_line(&mut b).unwrap();

    //     let a = a.trim().parse().unwrap();
    //     let b = b.trim().parse().unwrap();

    //     let sum = a + b;
    //     println!("the sum value is {}", sum);
    // }

    // if else

    // fn odd() {
    //     let mut a = String::new();
    //     io::stdin().read_line(&mut a).unwrap();
    //     let a: i32 = a.trim().parse().unwrap();

    //     if a % 2 == 0 {
    //     } else {
    //         println!("The number is oddNumber ");
    //     }
    // }

    // Largest of two

    // fn largest() {
    //     let mut a = String::new();
    //     let mut b = String::new();

    //     io::stdin().read_line(&mut a).unwrap();
    //     io::stdin().read_line(&mut b).unwrap();

    //     if a > b {
    //         println!("A is bigger than b");
    //     } else {
    //         println!("B is bigger than a ")
    //     }
    // }

    // POsitive Negative

    fn postive() {
        let mut a = String::new();
        io::stdin().read_line(&mut a).unwrap();
        let a: i32 = a.trim().parse().unwrap();

        if a > 0 {
            println!("Positive");
        }
        else if a < 0{
            println!("Negative");
        }
        else{
            println!("Zero")
        }
    }













    postive();
    // largest();
    // odd();
    // sumdigit();
    // print();
}
