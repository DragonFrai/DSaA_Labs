// #![feature(binary_heap_retain)]

mod find;
mod play15;

use crate::play15::Board;
use crate::play15::Dir;
use ndarray::Array2;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::ops::Add;
use std::rc::Rc;
use std::time::Duration;
use text_io::read;


#[derive(Debug, Deserialize)]
enum E {
    A,
    B,
}

fn main() {
    bench_find();
    find_board_solution();
}

fn bench_find() {
    let mut strings = Vec::new();
    let mut rng = rand::rngs::StdRng::from_entropy();
    crate::find::gen_rand_str_with_substr(100_000, &mut strings, &mut rng);

    fn bench_finder<'a>(
        mut f: impl FnMut(&'a str, &'a str) -> Option<usize>,
        strings: &'a [(String, String)],
    ) -> Duration {
        let start = std::time::Instant::now();
        for (s, pat) in strings {
            f(s, pat);
        }
        let end = std::time::Instant::now();
        end.duration_since(start)
    };

    println!("Кнута-Морриса-Пратта: ...");
    println!(
        "Total = {} сек\n",
        bench_finder(find::kmp_find, &strings).as_secs_f64()
    );
    println!("Бойера-Мура: ...");
    println!(
        "Total = {} сек\n",
        bench_finder(find::bm_find, &strings).as_secs_f64()
    );
    println!("Стандартный поиск: ...");
    println!(
        "Всего = {} сек\n",
        bench_finder(str::find, &strings).as_secs_f64()
    );
}

fn test_find() {
    let mut strings = Vec::new();
    let mut rng = rand::rngs::StdRng::from_entropy();
    crate::find::gen_rand_str_with_substr(10, &mut strings, &mut rng);
    strings.push(("123456".to_string(), "234".to_string()));
    for (s, pat) in &strings {
        println!("{}", s);
        println!("{}", pat);
        println!("{:?} --> {:?}", find::kmp_find(s, pat), s.find(pat));
        println!("{:?} --> {:?}", find::bm_find(s, pat), s.find(pat));

        println!();
    }
}

fn find_board_solution() {
    println!("> Пятнашки!");

    loop {
        // print!("Введите размеры доски: ");
        std::io::stdout().flush().unwrap();
        // let (w, h): (String, String) = (read!(), read!());
        // if w == "q" || h == "q" {
        //     println!("Выход из пятнашек.");
        //     break;
        // }
        // let (w, h): (usize, usize) = (w.parse().unwrap(), w.parse().unwrap());

        // if play15::check_array_size(w, h).is_err() {
        //     println!("Размеры доски слишком большие. На ней должно быть не более 255 плиток.");
        //     continue;
        // }

        println!(
            "Введите расположение {} плиток: ",
            play15::WIDTH * play15::HEIGHT
        );
        let mut arr = [[0u8; play15::WIDTH]; play15::HEIGHT];
        for y in 0..play15::HEIGHT {
            for x in 0..play15::WIDTH {
                arr[y][x] = read!();
            }
        }

        let board = match Board::from_array(arr) {
            Ok(b) => b,
            Err(_) => {
                println!("Такое расположение плиток невозможно!");
                continue;
            }
        };

        let can_solve = board.can_solve();
        println!(
            "{}",
            if can_solve {
                "Решение возможно. Решаю..."
            } else {
                "Решение невозможно."
            }
        );
        if can_solve {
            let solve = board.solve().unwrap();
            println!("Решение включает {} ходов.", solve.len());
            let mut board = board.clone();
            println!("Исходная: \n{}", &board);

            let mut dirs = Vec::new();
            let mut node = solve;
            while node.step.is_some() {
                let (last, dir) = {
                    let (last, dir) = node.step.as_ref().unwrap();
                    (Rc::clone(last), *dir)
                };
                node = last;
                dirs.push(dir);
            }

            for dir in dirs.into_iter().rev() {
                board.apply(dir);
                let dir_as_letter = |dir: Dir| match dir {
                    Dir::Up => "U",
                    Dir::Left => "L",
                    Dir::Right => "R",
                    Dir::Down => "D",
                };
                print!("{}", dir_as_letter(dir));
                //println!("{}", board);
            }
        }
        println!();
    }
}

fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    todo!()
}
