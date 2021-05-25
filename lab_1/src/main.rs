mod n1;
mod n2;
mod n3;
mod rng;

pub const VARIANT: i32 = 19;

fn main() {
    let a = 2;
    let b = 12;

    let print_matrix = false;

    let mut matrix = n2::gen_matrix(200, 5000, -50000, 50000);
    println!("Исходная матрица");
    if print_matrix {
        n2::print_matrix(&matrix);
    }

    let start = std::time::Instant::now();
    n3::sort_matrix(&mut matrix, |arr| arr.sort());
    println!("Сортировка стандартной библиотеки");
    println!(
        "{} ms",
        std::time::Instant::now().duration_since(start).as_millis()
    );
    if print_matrix {
        n2::print_matrix(&matrix);
    }

    let start = std::time::Instant::now();
    n3::sort_matrix(&mut matrix, n3::select_sort);
    println!("Сортировка выбором");
    println!(
        "{} ms",
        std::time::Instant::now().duration_since(start).as_millis()
    );
    if print_matrix {
        n2::print_matrix(&matrix);
    }

    let start = std::time::Instant::now();
    n3::sort_matrix(&mut matrix, n3::insert_sort);
    println!("Сортировка вставками");
    println!(
        "{} ms",
        std::time::Instant::now().duration_since(start).as_millis()
    );
    if print_matrix {
        n2::print_matrix(&matrix);
    }

    let start = std::time::Instant::now();
    n3::sort_matrix(&mut matrix, n3::bubble_sort);
    println!("Сортировка пузырьковая");
    println!(
        "{} ms",
        std::time::Instant::now().duration_since(start).as_millis()
    );
    if print_matrix {
        n2::print_matrix(&matrix);
    }

    let start = std::time::Instant::now();
    n3::sort_matrix(&mut matrix, n3::shell_sort);
    println!("Сортировка Шелла");
    println!(
        "{} ms",
        std::time::Instant::now().duration_since(start).as_millis()
    );
    if print_matrix {
        n2::print_matrix(&matrix);
    }

    let start = std::time::Instant::now();
    n3::sort_matrix(&mut matrix, n3::heap_sort);
    println!("Сортировка пирамидальная");
    println!(
        "{} ms",
        std::time::Instant::now().duration_since(start).as_millis()
    );
    if print_matrix {
        n2::print_matrix(&matrix);
    }

    let start = std::time::Instant::now();
    n3::sort_matrix(&mut matrix, n3::quick_sort);
    println!("Сортировка быстрая");
    println!(
        "{} ms",
        std::time::Instant::now().duration_since(start).as_millis()
    );
    if print_matrix {
        n2::print_matrix(&matrix);
    }
}
