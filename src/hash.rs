use std::collections::HashMap;

// use crate::pratice;

pub fn hash() {
    // let mut student_marks = HashMap::new();

    // student_marks.insert("Math", 95);
    // student_marks.insert("English", 88);
    // student_marks.insert("Rust", 100);

    // for (subject, mark) in student_marks {
    //     println!("The student  of {subject} and {mark}")
    // }

    // fn capit() {
    //     let mut country_capital = HashMap::new();
    //     country_capital.insert("India", "Delhi");
    //     country_capital.insert("USA", "Washington");
    //     country_capital.insert("France", "Paris");

    //     match country_capital.get("India") {
    //         Some(value) => println!("Delhi is hte capital of {value}"),
    //         None => println!("None of the value find "),
    //     }

    //     for (country, capital) in country_capital {
    //         println!("The country is {country} and its capital is {capital}")
    //     }
    // }

    // fn pratice1(){
    //     let mut fruit = HashMap::new();
    //     fruit.insert("Apple" , 100);
    //     fruit.insert("Mango" , 200);
    //     fruit.insert("Chiku" , 50);

    //     println!("{:?}" , fruit)
    // }

    // fn pratice2(){
    //       let mut fruit = HashMap::new();
    //     fruit.insert("Apple" , 100);
    //     fruit.insert("Mango" , 200);
    //     fruit.insert("Chiku" , 50);

    //     match fruit.get("Mango") {
    //         Some(value) => println!("the value of Mango is {value}"),
    //         None => println!("not Found")
    //     }

    // }

    // fn pratice3() {
    //     let mut fruit = HashMap::new();

    //     fruit.insert("Apple", 100);
    //     fruit.insert("Mango", 200);
    //     fruit.insert("Chiku", 50);

    //     for (fruit, price) in &fruit {
    //         println!("The fruit is {fruit} and its price is {price}")
    //     }
    // }

    // fn pratice4() {
    //     let mut marks = HashMap::new();

    //     marks.insert("English", 200);
    //     marks.insert("math", 100);
    //     marks.insert("physics", 50);

    //     println!("the length of the marks is : {}" , marks.len())
    // }

    fn pratice5() {
        let mut marks = HashMap::new();
        marks.insert("English", 200);
        marks.insert("math", 100);
        marks.insert("physics", 50);

        if marks.contains_key("PhysicsWala") {
            println!("Key is present ")
        } else {
            println! {"Chud Gaye Guru"
            }
        }
    }

    pratice5();
    // pratice4();
    // pratice3();
    // pratice2();
    // pratice1();
    // capit();
}
