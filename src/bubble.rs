pub fn bubble() {
    fn bubblealg(arr: &mut [i32]) {
        let n = arr.len();

        for i in 0..n {
            for j in 0..n - 1 - i {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1)
                }
            }
        }
    }

    fn callbuble() {
        let mut number = [1, 3, 6, 2, 4, 5];

        bubblealg(&mut number);
        println!("{:?}", number)
    }

    callbuble();
}

fn bubblealg(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            arr.swap(j, j + 1);
        }
    }
}
