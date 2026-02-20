pub fn borrow() {
    // fn borrow_task () {

    //     let mut number = vec![1, 2, 3, 4, 5, 6];

    //     for count in number.iter_mut() {
    //         *count = *count * 10;
    //     }

    //     println!("{:?}", number);
    // }

    // fn complete_borrow () {
    //     let mut number = vec![1,2,4,5,6,7,8,9];

    //     for x in number {
    //         println!("{}" ,x )
    //     }

    //     // println!("{:?}" , number)
    // }

    fn task_borrow() {
        let  number = vec![10, 20, 30];

        for x in &number {
            println!("{:?}", x)
        }

        println!("{:?}", number);
    }

    task_borrow();

    // complete_borrow();
    // borrow_task();
}
