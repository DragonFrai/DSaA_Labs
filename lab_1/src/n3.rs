pub fn sort_matrix<F: FnMut(&mut [i32])>(matrix: &mut Vec<Vec<i32>>, mut sorter: F) {
    matrix.into_iter().for_each(|x| sorter(x.as_mut_slice()))
}

/// Сортировка выбором
pub fn select_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..(arr.len() - 1) {
        let min_idx = {
            let mut min_idx = i;
            let mut min_val = &arr[i];
            for i in (i + 1)..arr.len() {
                let x = &arr[i];
                if x < min_val {
                    min_idx = i;
                    min_val = x;
                }
            }
            min_idx
        };
        arr.swap(i, min_idx)
    }
}

/// Сортировка вставками
pub fn insert_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut n = i;
        while n > 0 && arr[n] < arr[n - 1] {
            arr.swap(n, n - 1);
            n -= 1;
        }
    }
}

/// Сортировка Шелла
pub fn shell_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    let mut s = len / 2;
    while s > 0 {
        for i in 0..len {
            for j in ((i + s)..len).step_by(s) {
                if arr[i] > arr[j] {
                    arr.swap(i, j);
                }
            }
        }
        s /= 2;
    }
}

/// Сортировка пузырьковая
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for lim in (0..arr.len()).rev() {
        for i in 0..lim {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1)
            }
        }
    }
}

// Heapsort
pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    // Преобразуем массив в сортирующее дерево
    let end = arr.len();
    // Пропускаем последние end/2 элементов (листы дерева не имеет смысла перемещать между собой)
    for start in (0..end / 2).rev() {
        sift_down(arr, start, end - 1);
    }

    // Сортировка сортирующего дерева
    for end in (1..arr.len()).rev() {
        // Самый большой элемент считаем отсортированным. Перемещаем его в конец
        // и исключаем из дерева
        arr.swap(end, 0);
        // "всплывает" следующий наибольший элемент
        sift_down(arr, 0, end - 1);
    }
}

// "поднимаем" наибольший элемент из дочерних в позицию start
fn sift_down<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1; // Получаем левого ребенка
        if child > end {
            break;
        }
        if child < end && arr[child] < arr[child + 1] {
            // Если правый ребенок существует и больший
            child += 1;
        }

        if arr[root] < arr[child] {
            // Если ребенок не меньше корня, меняем их
            arr.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}

/// Сортировка турнирная
pub fn turn_sort<T: Ord>(arr: &mut [T]) {
    unimplemented!()
}

/// Сортировка быстрая
pub fn quick_sort<T: Ord + Copy>(arr: &mut [T]) {
    let vec = arr.iter().map(|x| *x).collect::<Vec<_>>();
    let result = quick_sort_inner(vec);
    for i in 0..result.len() {
        arr[i] = result[i]
    }
}

pub fn quick_sort_inner<T: Ord + Copy>(arr: Vec<T>) -> Vec<T> {
    if arr.len() <= 1 {
        return arr;
    }

    let mut less = Vec::new();
    let mut equal = Vec::new();
    let mut great = Vec::new();

    let cmpr = arr[arr.len() / 2];

    for x in arr {
        if x < cmpr {
            less.push(x);
        }
        if x == cmpr {
            equal.push(x);
        }
        if x > cmpr {
            great.push(x);
        }
    }

    let mut less = quick_sort_inner(less);
    let mut great = quick_sort_inner(great);

    less.append(&mut equal);
    less.append(&mut great);

    less
}

pub fn run() {}
