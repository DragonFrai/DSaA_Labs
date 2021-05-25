use ndarray::Array2;
use rand::rngs::StdRng;
use std::ops::RangeFrom;
use rand::{SeedableRng, Rng};

pub fn matrix() {
    let n = 5;
    let m = 10;

    let mut rng = StdRng::from_entropy();
    let mut mat = Array2::from_shape_fn([m, n], |_| rng.gen_range(10..100));

    print_matrix(&mat, "Исходная");

    sort_matrix(&mut mat);

    print_matrix(&mat, "Отсортированная");

}

fn sort_matrix(mat: &mut Array2<i32>) {
    let m = mat.shape()[0];
    let n = mat.shape()[1];

    for d_i in 0..(m + n - 1) {
        let (start_x, start_y) = if d_i < n { (0, n - d_i - 1) } else { (d_i - n, 0) };

        let mut arr = Vec::new();

        for offset in 0.. {
            let (x, y) = (start_x + offset, start_y + offset);
            if x >= m || y >= n { break }
            arr.push(mat[(x, y)]);
        }

        arr.sort_by(|a, b| b.cmp(a));

        for offset in 0.. {
            let (x, y) = (start_x + offset, start_y + offset);
            if x >= m || y >= n { break }
            mat[(x, y)] = arr.pop().unwrap();
        }
    }
}

fn print_matrix(mat: &Array2<i32>, m: &str) {
    println!("{}:", m);
    for i in 0..mat.shape()[1] {
        for j in 0..mat.shape()[0] {
            print!(" {}", mat[(j, i)]);
        }
        println!()
    }
}