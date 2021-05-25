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

use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

pub fn make_max_mum(nums: &mut [i32]) -> String {
    let mut str = String::new();
    nums.sort_by(|a, b| Ord::cmp(&concat(*b, *a), &concat(*a, *b)));
    for x in nums {
        // В случае, если к нулю добавляется число, очищаем строку (убираем незначащий ноль)
        if str == "0" { str.truncate(0); }
        write!(&mut str, "{}", x).unwrap(); // добавление числа к не нулевой или пустой строке
    }
    str
}

// Конкатинирует 2 числа как если бы это были строки
fn concat(n1: i32, n2: i32) -> i64 {
    let (n1, n2) = (n1 as i64, n2 as i64);
    let offset = pow10ceil(n2);
    n1*offset + n2 // return n1*offset + n2
}

// Округляет по степени 10 вверх (0 -> 10; 2 -> 10; 10 -> 100; 24 -> 100)
fn pow10ceil(mut n: i64) -> i64 {
    let mut pow = 10;
    while n >= 10 {
        n /= 10;
        pow *= 10;
    }
    pow // return pow
}
