use crate::rng::Rng;
use std::io::BufRead;
use std::time::UNIX_EPOCH;

const DEFAULT_M: usize = 50;
const DEFAULT_N: usize = 50;

const DEFAULT_MIN: i32 = -250;
const DEFAULT_MAX: i32 = 1000 + crate::VARIANT;

fn parse_input(inp: &str) -> Result<(usize, usize, i32, i32), ()> {
    let comp = inp.split_whitespace().collect::<Vec<_>>();
    match comp.as_slice() {
        [m, n, min, max] => Ok((
            m.parse().map_err(|_| ())?,
            n.parse().map_err(|_| ())?,
            min.parse().map_err(|_| ())?,
            max.parse().map_err(|_| ())?,
        )),
        _ => Err(()),
    }
}

fn read_input() -> Result<(usize, usize, i32, i32), ()> {
    let mut buf = String::new();
    let _read = std::io::stdin()
        .lock()
        .read_line(&mut buf)
        .map_err(|_| ())?;

    if buf == "" || buf == "\n" {
        return Ok((DEFAULT_M, DEFAULT_N, DEFAULT_MIN, DEFAULT_MAX));
    }

    parse_input(&buf)
}

pub fn gen_matrix(m: usize, n: usize, min: i32, max: i32) -> Vec<Vec<i32>> {
    let mut rng = Rng::new(
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|dur| dur.as_secs())
            .unwrap_or(173),
    );

    (0..m)
        .map(|_| (0..n).map(|_| rng.gen_i32_in(min, max)).collect())
        .collect()
}

pub fn print_matrix(matrix: &Vec<Vec<i32>>) {
    matrix.into_iter().for_each(|line| {
        line.into_iter().for_each(|x| print!("{} ", x));
        println!()
    });
    println!()
}

pub fn run() {
    println!("Введите m, n, min, max через пробел или нажмите Enter: ");

    let mut input = None;
    while input.is_none() {
        let read_result = read_input();
        match read_result {
            Ok(x) => {
                let (_m, _n, min, max) = x;
                if min <= max {
                    input = Some(x);
                } else {
                    println!("Значение min > max. Попробуйте ещё раз: ");
                }
            }
            Err(_) => println!("Некорректный ввод. Попробуйте еще раз: "),
        }
    }
    let (m, n, min, max) = input.unwrap();

    let matrix = gen_matrix(m, n, min, max);

    println!("Сгенерированнаяя матрица: ");
    print_matrix(&matrix)
}
