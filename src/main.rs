// fn main() {
//     println!("Hello, world!");
//     println!("Learning rust from the scrastsh ");
//     println!("starting wiht the new rust program");
// }

// fn main (){
//     let x = 21;
//     println!("the value of x is {}" , x )
// }

// variable are immutable by default in rust we you want to change the variable then we need to provide the "mut" keyword in the front of the variable

// fn main() {
//     let mut x = 20;
//     println!("the value of x is {}", x);

//     x = 22;
//     println!("the value of x is {}", x);
// }

// Lesson 3 : Constant vs variable

// fn main (){
//     const PI:f64 = 3.1415;
//     println!("the value of PI is {}" , PI);

// }

// lesson  4 : Shadowing

// fn main (){
//     let x = 5;
//     println!("The value of x is {}" , x );

//     let x = x + 1;
//     println!("The value of x is {}" , x);

//     let x = x + 3 ;
//     println!("The value of x is {}", x)

// }

// fn main(){
//     const MAX_SCORE: i32 = 100;

//     let score = 50;
//     println!("The value of score is {}" , score);

//     let score = score + 20;
//     println!("The value of score is {}" , score);

//     println!("The value of MAX_SCORE is {}", MAX_SCORE);
// }

// fn main() {
//     let age: i32 = 20;
//     println!("age is {}", age);

//     let height: f64 = 200.6;
//     println!("Height is {}", height);

//     let is_student: bool = true;
//     println!("is student is {}", is_student);

//     let grade: char = 'B';
//     println!("Grade is {}", grade);

//     let info: (&str, i32) = ("person", 20);
//     println!("name is {}", info.0);
//     println!("age is {}", info.1);

//     let number: [i32; 5] = [1, 2, 3, 4, 5];
//     println!("number is {}", number[0]);
//     println!("number is {}", number[1]);
//     println!("number is {}", number[2]);
//     println!("number is {}", number[3]);
//     println!("number is {}", number[4]);
// }

// string in RUST

// fn main(){
//     let name: &str  = "Rust";
//     println!("This is rust {}", name);

//     let mut name: String  = String::from("Rust");
//     name.push_str(      " Language");
//     println!("{}", name)

// }

// fn main() {
//     let language: &str = "Rust";
//     println!("langge at the start is {}", language);

//     let mut course: String = String::from("learning");
//     course.push_str( " Rust");
//     println!("{}", course);
// }

// use std::io;
// fn main() {
//     let mut name = String::new();
//     let mut age = String::new();
//     let mut height = String::new();

//     println!("Enter your name ",);
//     io::stdin()
//         .read_line(&mut name)
//         .expect("failed to read the line");

//     println!("Enter your age ",);
//     io::stdin()
//         .read_line(&mut age)
//         .expect("failed to read the line");

//     println!("Enter your height ",);
//     io::stdin().read_line( &mut height).expect("This line is not readbable");

//     println!("NAme is {}", name);

//     println!("NAme is {}", age);
//     println!("Height is {}", height)
// }

// use std::io;

// fn main() {
//     let mut a = String::new();
//     println!("Enter the value of a");

//     let mut b = String::new();
//     println!("Enter the value of b");

//     io::stdin()
//         .read_line(&mut a)
//         .expect("failed to read the line");
//     io::stdin()
//         .read_line(&mut b)
//         .expect("failed to read the line");

//     let a: i32 = a.trim().parse().unwrap();
//     let b: i32 = b.trim().parse().unwrap();

//     println!("a + b = {}", a + b);
//     println!("a - b = {}", a - b);
//     println!("a * b = {}", a * b);
//     println!("a % b = {}", a % b);
//     println!("a / b = {}", a / b);
// }



fn main(){
    let age: i32 = 13;
    if age < 13{
        println!("Your are child");
    }
    else if age >= 13 && age <= 19 {
        println!("Your are Teen");
    }
    else {
        println!("Your are adult")
    }
}
