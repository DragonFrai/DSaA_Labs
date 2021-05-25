use rand::rngs::SmallRng;
use rand::{SeedableRng, Rng};

fn tri_exists(a: i32, b: i32, c: i32) -> bool {
    let max = a.max(b).max(c);
    let min = a.min(b).min(c);
    let mid = a + b + c - max - min;
    max < (min + mid)
}

fn max_p(arr: &mut [i32]) -> Option<(i32, i32, i32)> {

    arr.sort();

    for i in (2..arr.len()).rev() {
        let a = arr[i];
        let b = arr[i - 1];
        let c = arr[i - 2];
        if tri_exists(a, b, c) {
            return Some((a, b, c));
        }
    }
    None
}

pub fn tri() {
    let mut rng = SmallRng::from_entropy();
    let length = 20;
    let mut arr = (0..length).map(|_| rng.gen_range(1..100)).collect::<Vec<_>>();

    // arr = vec![2, 1, 2];

    println!("Исходный массив: {:?}", &arr);

    let res = max_p(&mut arr);

    if let Some((a, b, c)) = res {
        println!("Наибольий периметр = {}", a+b+c);
        return;
    } else {
        println!("0");
    }
}