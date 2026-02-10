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


fn main(){
    const MAX_SCORE: i32 = 100;
    
    let score = 50;
    println!("The value of score is {}" , score);
    
    let score = score + 20;
    println!("The value of score is {}" , score);
    
    
    println!("The value of MAX_SCORE is {}", MAX_SCORE);
}