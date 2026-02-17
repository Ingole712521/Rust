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

// fn main(){
//     let age: i32 = 13;
//     if age < 13{
//         println!("Your are child");
//     }
//     else if age >= 13 && age <= 19 {
//         println!("Your are Teen");
//     }
//     else {
//         println!("Your are adult")
//     }
// }

// fn main(){
//     let number  = 6;
//     match number {
//         1 => println!("Monday"),
//         2 => println!("Tuesday"),
//         3 => println!("Wednesday"),
//         4 => println!("Thrusday"),
//         5 => println!("Friday"),
//         6 => println!("Saturday"),
//         7 => println!("Sunday"),
//         _ => println!("Invalid number")
//     }

// }

// fn main(){
//     let mut number = 1;

//     while number <=10 {
//         println!("Number is {}", number);
//     number +=1;
//     }
// }

// fn main (){
//     for number in 1..10{
//         println!("Number is {}", number);
//     }
// }

// fn main() {
//     for number in 1..=20 {
//         if number % 2 == 0 {
//             println!("Number is even {}", number);
//         }
//     }
// }

// Function in rust

// fn multiply(a: i32, b: i32)-> i32 {
//    return a * b
// }

// fn main(){
//     let result= multiply(5,10);
//     println!("THe result is {}", result);

// }

// Expressions vs Statements

// fn main(){
//     let number = 7;

//     let result = if number %2 ==0 {
//         0
//     }
//     else{
//          1
//     };
//     println!("The result is {}", result)
// }

// fn main() {
//     let mut name = String::from("Rust");
//     let changeName = &mut name;
//     changeName.push_str( " Language");

//     println!("Changed Name is {}", changeName)
// }

// fn print_length(s: &String){
//     println!("Length of the sting is {}" , s.len());
// }

// fn main(){

//     let result = String::from("Text");
//     print_length(&result);

// }

// fn add_suffix(s: &mut String) {
//     s.push_str("Text Add");
//     println!("Result is {}" , s)
// }

// fn main() {
//     let mut result = String::from(" Learning");
//     add_suffix(&mut result);
// }

// fn create_message() -> String{
//     let s = String::from("Hello Solana ");
//     return s
// }

// fn main(){
//     let message = create_message();
//     println!("Message is {}", message);
// }

// fn main(){
//     let s = String::from("SolanaLearing");
//     let result = &s[0..6];
//     println!("Result is {}", result)
// }

// fn main(){
//     let arr = [1,2,3,4,5,6,7,8,9,10];
//     let slice = &arr[3..7];
//     println!("The slice of the array is {:?}", slice);
// }

// struct User {
//     name: String,
//     roll_no: i32,
//     is_active: bool,
// }

// fn main() {
//     let user_new = User {
//         name: String::from("Hello"),
//         roll_no: 25,
//         is_active: false,
//     };

//     println!("The name is new user is {} ", user_new.name);
//     println!("The age is new user is {} ", user_new.roll_no);
//     println!("The active is new user is  {} ", user_new.is_active);
// }

// struct Student {
//     name: String,
//     roll_no: i32,
// }

// impl Student {
//     fn display(&self) {
//         println!("Name is {}", self.name);
//         println!("ROll no is {}", self.roll_no);
//     }
// }

// fn main() {
//     let s1 = Student {
//         name: String::from("OMPL"),
//         roll_no: 10,
//     };
//     s1.display();
// }

// struct Car {
//     brand: String,
//     year: i32,
// }

// impl Car {
//     fn display(&self) {
//         println!("Brand is {}", self.brand);
//         println!("Year is {}", self.year)
//     }
// }

// fn main() {
//     let c1 = Car {
//         brand: String::from("KIA"),
//         year: 200001,
//     };
//     c1.display();
// }

// enum PaymentMethod {
//     Cash,
//     Card,
//     Crypto,
// }

// fn main(){
//     let pay = PaymentMethod::Card;
//     match pay{
//         PaymentMethod::Card => println!("Paid using Card"),
//         PaymentMethod::Cash => println!("Paid using Cash"),
//         PaymentMethod::Crypto => println!("Paid using crypto"),
//     }
// }

// pub mod other_file;
mod circle;
mod input;
mod matching;
mod pratice;
mod vectors;
mod hash;
// mod option;
mod handling;

fn main() {
    pratice::pratice();
    input::input();
    matching::matching();
    circle::circle();
    vectors::vectors();
    hash::hash();
    // option::option();
    handling::handling();
}
