use std::cmp::Ordering;
use std::hash::Hash;


// --------------
// Бинарный поиск
// --------------
pub fn binary_search<T: PartialOrd>(arr: &[T], el: &T) -> Option<usize> {
    let mut left = 0usize;
    let mut right = arr.len() - 1;
    loop {
        if left > right || right == usize::MAX { return None }
        let mid = left + (right - left) / 2;
        if &arr[mid] < el { left = mid + 1; }
        if &arr[mid] > el { right = mid.wrapping_sub(1); }
        if &arr[mid] == el { return Some(mid); }
    }
}

// ----------------
// Интерполяционный
// ----------------
pub fn interpolation_search(sorted_array: &[i32], to_find: i32) -> Option<usize> {
    // Возвращает индекс элемента со значением to_find или None, если такого элемента не существует
    let mut low = 0usize;
    let mut high = sorted_array.len() - 1;

    while sorted_array[low] < to_find && sorted_array[high] > to_find {
        if sorted_array[high] == sorted_array[low] {// Защита от деления на 0
            break;
        }

        let mid = low + (((to_find - sorted_array[low]) as usize) * (high - low)) / ((sorted_array[high] - sorted_array[low]) as usize);

        match sorted_array[mid].cmp(&to_find) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid - 1,
            Ordering::Equal => return Some(mid),
        }
    }

    if sorted_array[low] == to_find {
        Some(low)
    } else if sorted_array[high] == to_find {
        Some(high)
    } else {
        None
    }
}

// -----------------
// Фибонначиев поиск
// -----------------
pub fn fibonacci_search(lys: &[i32], val: i32) -> Option<usize> {
    let mut fib_m_minus_2 = 0i32;
    let mut fib_m_minus_1 = 1i32;
    let mut fib_m = fib_m_minus_1 + fib_m_minus_2;
    while (fib_m as usize) < lys.len() {
        fib_m_minus_2 = fib_m_minus_1;
        fib_m_minus_1 = fib_m;
        fib_m = fib_m_minus_1 + fib_m_minus_2;
    }
    let mut index = -1;
    while fib_m > 1 {
        let i = usize::min((index + fib_m_minus_2) as usize, lys.len() - 1);
        if lys[i] < val {
            fib_m = fib_m_minus_1;
            fib_m_minus_1 = fib_m_minus_2;
            fib_m_minus_2 = fib_m - fib_m_minus_1;
            index = i as i32;
        }
        else if lys[i] > val {
            fib_m = fib_m_minus_2;
            fib_m_minus_1 = fib_m_minus_1 - fib_m_minus_2;
            fib_m_minus_2 = fib_m - fib_m_minus_1;
        }
        else {
            return Some(i);
        }
    }
    if (index as usize) < (lys.len()-1) && lys[(index as usize)+1] == val {
        return Some(index as usize + 1);
    }
    return None;
}

// -------------------------
// Поиск по бинарному дереву
// -------------------------
#[derive(Debug)]
pub struct Node<T> {
    pub left: Option<Box<Node<T>>>,
    pub value: T,
    pub right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node { left: None, value, right: None, }
    }

    fn add_to_node(&mut self, value: T) where T: Ord {
        #[inline]
        fn add_optionally<T: Ord>(node: &mut Option<Box<Node<T>>>, value: T) {
            match node {
                Some(boxed) => boxed.add_to_node(value),
                None => *node = Some(Box::new(Node::new(value))),
            }
        }
        match value.cmp(&self.value) {
            Ordering::Less => add_optionally(&mut self.left, value),
            Ordering::Greater => add_optionally(&mut self.right, value),
            Ordering::Equal => ()/* Значение уже существует */,
        }
    }

    pub fn has_value(&self, value: &T) -> bool where T: Ord {
        match value.cmp(&self.value) {
            Ordering::Less => match &self.left {
                Some(node) => node.has_value(value),
                None => false,
            },
            Ordering::Greater => match &self.right {
                Some(node) => node.has_value(value),
                None => false,
            },
            Ordering::Equal => true,
        }
    }

}


#[derive(Debug)]
pub struct MyBinaryTree<T> {
    root: Option<Node<T>>,
}

impl<T> MyBinaryTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn add(&mut self, value: T) where T: Ord {
        if let Some(root) = &mut self.root {
            root.add_to_node(value);
        } else {
            self.root = Some(Node::new(value));
        }
    }

    pub fn has_value(&self, value: &T) -> bool where T: Ord {
        if let Some(root) = &self.root {
            root.has_value(value)
        } else {
            false
        }
    }
}
