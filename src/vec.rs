pub fn vec() {
    fn pratice1() {
        let mut a: Vec<String> = Vec::new();

        a.push(String::from("Rust"));
        a.push(String::from("Solana"));
        a.push(String::from("Blockchain"));

        for a in &a {
            println!("{}", a)
        }

    }
    pratice1();
}
