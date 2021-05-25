use rand::rngs::StdRng;
use rand::prelude::*;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub fn max() {
    let mut arr_src = gen_rand_array(15);
    let mut arr = arr_src.clone();

    // Измеряем время сортировки
    let start = Instant::now();
    let num = make_max_mum(&mut arr);
    let total = Instant::now().duration_since(start);

    // println!("Time: {:?}", &total);
    println!("Исходный массив: {:?}", &arr_src);
    println!("Число: {}", num);
}

// Генерация массива случайных чисел
fn gen_rand_array(len: usize) -> Vec<i32> {
    let mut rng = StdRng::from_entropy();
    let mut arr = Vec::new();
    for _i in 0..len {
        arr.push(rng.gen_range(0..1000));
    }
    arr
}

// Создает наибольшее число из массива чисел  в диапазоне [0; 999]
fn make_max_mum(arr: &mut [i32]) -> String {
    let mut str = String::new();
    arr.sort_by(|a, b| measure(*b).cmp(&measure(*a)));
    for x in arr {
        str += &format!("{}", x);
    }
    str
}

// Функция вычисления "меры" ценности числа
// 9 -> 999
// 78 -> 788
// 942 -> 942
fn measure(n: i32) -> i32 {
    if n < 10 { 100*n + 10*n + n }
    else if n < 100 { 10*n + n%10 }
    else if n < 1000 { n }
    else { unreachable!() }
}
