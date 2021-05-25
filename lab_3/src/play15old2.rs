use ndarray::Array2;
use rand::Rng;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};

pub const WIDTH: usize = 4;
pub const HEIGHT: usize = 4;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Board([[u8; WIDTH]; HEIGHT]);

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct BoardCreateError;

impl Board {
    pub fn new() -> Self {
        let mut arr = [[0u8; WIDTH]; HEIGHT];
        for y in 0..WIDTH {
            for x in 0..HEIGHT {
                arr[y][x] = ((y * WIDTH + x + 1) % (WIDTH * HEIGHT)) as u8
            }
        }
        Board(arr)
    }

    pub fn from_array(arr: [[u8; WIDTH]; HEIGHT]) -> Result<Self, BoardCreateError> {
        let w = WIDTH;
        let h = HEIGHT;
        let mut tile_count = vec![0; w * h];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                tile_count.get_mut(arr[y][x] as usize).map(|x| *x += 1);
            }
        }
        let has_one_of_all = tile_count.iter().all(|x| *x == 1);
        if has_one_of_all {
            Ok(Board(arr))
        } else {
            Err(BoardCreateError)
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (WIDTH, HEIGHT)
    }

    pub fn empty_at(&self) -> (usize, usize) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.0[y][x] == 0 {
                    return (x, y);
                }
            }
        }
        panic!()
    }

    #[inline(always)]
    pub fn swap(&mut self, p1: (usize, usize), p2: (usize, usize)) {
        let arr = &mut self.0;
        let t1 = arr[p1.1][p1.0];
        let t2 = arr[p2.1][p2.0];
        arr[p1.1][p1.0] = t2;
        arr[p2.1][p2.0] = t1;
    }

    pub fn apply(&mut self, dir: Dir) -> Result<(), ()> {
        let (zx, zy) = self.empty_at();
        let (w, h) = self.size();
        match dir {
            Dir::Right if zx < w - 1 => {
                self.swap((zx, zy), (zx + 1, zy));
                Ok(())
            }
            Dir::Down if zy < h - 1 => {
                self.swap((zx, zy), (zx, zy + 1));
                Ok(())
            }
            Dir::Left if zx > 0 => {
                self.swap((zx, zy), (zx - 1, zy));
                Ok(())
            }
            Dir::Up if zy > 0 => {
                self.swap((zx, zy), (zx, zy - 1));
                Ok(())
            }
            _ => Err(()),
        }
    }

    //
    // pub fn possible_steps_with<F: FnMut(Board, u8)>(&self, mut f: F) {
    //     let (zx, zy) = self.empty_at();
    //     let w = self.0.shape()[0];
    //     let h = self.0.shape()[1];
    //     if zx < w - 1 {
    //         // Направо
    //         let mut b = self.clone();
    //         b.0.swap((zx, zy), (zx + 1, zy));
    //         let moved = b.0[(zx, zy)];
    //         f(b, moved)
    //     }
    //     if zy < h - 1 {
    //         // Вниз
    //         let mut b = self.clone();
    //         b.0.swap((zx, zy), (zx, zy + 1));
    //         let moved = b.0[(zx, zy)];
    //         f(b, moved)
    //     }
    //     if zx > 0 {
    //         // Налево
    //         let mut b = self.clone();
    //         b.0.swap((zx, zy), (zx - 1, zy));
    //         let moved = b.0[(zx, zy)];
    //         f(b, moved)
    //     }
    //     if zy > 0 {
    //         // Вверх
    //         let mut b = self.clone();
    //         b.0.swap((zx, zy), (zx, zy - 1));
    //         let moved = b.0[(zx, zy)];
    //         f(b, moved)
    //     }
    // }

    pub fn is_solved(&self) -> bool {
        let (w, h) = self.size();
        for y in 0..h {
            for x in 0..w {
                if ((y * w + x + 1) % (w * h)) as u8 != self.0[y][x] {
                    return false;
                }
            }
        }
        true
    }

    pub fn can_solve(&self) -> bool {
        // let (w, h) = self.size();
        // let mut flat = Vec::<u8>::with_capacity(w * h);
        // for y in 0..h {
        //     for x in 0..w {
        //         flat.push(self.0[(x, y)]);
        //     }
        // }
        // let (_zx, zy) = self.empty_at();
        // let sum: usize = (0..flat.len())
        //     .map(|i| {
        //         let c = flat[i] as usize;
        //         let c = if c == 0 { w * h } else { c };
        //         let k = flat[i..]
        //             .iter()
        //             .map(|x| if *x == 0 { (w * h) } else { *x as usize })
        //             .filter(|x| *x < c)
        //             .count();
        //         k
        //     })
        //     .sum();
        // let n = sum + zy;
        // n % 2 == 0
        true
    }

    pub fn wrong_tiles(&self) -> usize {
        let (w, h) = self.size();
        let mut c = 0;
        for y in 0..h {
            for x in 0..w {
                if ((y * w + x + 1) % (w * h)) as u8 != self.0[y][x] {
                    c += 1;
                }
            }
        }
        c
    }

    pub fn solve(&self) -> Result<Path, ()> {
        if !self.can_solve() {
            return Err(());
        }

        let mut checked_position_length = HashMap::new();
        let mut heap = BinaryHeap::with_capacity(1000);

        heap.push(QPath(Path::new(self.clone())));

        let mut i = 0;
        loop {
            i += 1;
            let current = heap.pop().unwrap();
            let last = checked_position_length.get_mut(&current.0.current_board);
            let remove_longer = |heap: &mut BinaryHeap<QPath>, to_remove: Board| {
                heap.retain(|qpath| qpath.0.current_board != to_remove);
            };

            if i % 10_000 == 0 {
                println!(
                    "iter = {}e4, path len = {}, euristic = {}, in heap {} el",
                    i / 10_000,
                    current.0.path().len(),
                    current.0.current_board().wrong_tiles(),
                    heap.len()
                );
            }

            match last {
                Some(last) if *last <= current.0.path.len() => continue,
                Some(last) => {
                    *last = current.0.path.len();
                    //remove_longer(&mut heap, current.0.current_board);
                }
                _ => {
                    checked_position_length.insert(current.0.current_board, current.0.path.len());
                    //remove_longer(&mut heap, current.0.current_board);
                }
            }

            // println!("Current board with {}", current.cost());
            if current.0.current_board().is_solved() {
                return Ok(current.0);
            }
            let mut push_or_ignore = |dir| {
                // Oh... Remove?
                if heap.len() > 1_000_000 {
                    let mut replacement = BinaryHeap::with_capacity(1_000_005);
                    for _i in 0..10_000 {
                        replacement.push(heap.pop().unwrap());
                    }
                    heap = replacement;
                }
                // ^^^^^^^
                let mut c = &current;
                let path = c.0.push_step_cloned(dir);
                if let Ok(path) = path {
                    if !checked_position_length.contains_key(path.current_board()) {
                        heap.push(QPath::new(path));
                    }
                }
            }; // 15 2 1 12 8 5 6 11 4 9 10 7 3 14 13 0
            push_or_ignore(Dir::Up);
            push_or_ignore(Dir::Right);
            push_or_ignore(Dir::Down);
            push_or_ignore(Dir::Left);
        }
    }

    pub fn inner(&self) -> &[[u8; WIDTH]; HEIGHT] {
        &self.0
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (w, h) = self.size();
        for y in 0..h {
            for x in 0..w {
                match w * h {
                    0..=9 => write!(f, "{:1} ", self.0[y][x])?,
                    10..=99 => write!(f, "{:2} ", self.0[y][x])?,
                    100..=999 => write!(f, "{:3} ", self.0[y][x])?,
                    _ => panic!(""),
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Path {
    current_board: Board,
    path: Vec<Dir>,
}

impl Path {
    pub fn current_board(&self) -> &Board {
        &self.current_board
    }
    pub fn path(&self) -> &Vec<Dir> {
        &self.path
    }
    pub fn len(&self) -> usize {
        self.path.len()
    }
}

impl Path {
    pub fn new(start_board: Board) -> Self {
        Self {
            current_board: start_board,
            path: Vec::new(),
        }
    }

    pub fn push_step(&mut self, dir: Dir) -> Result<(), ()> {
        self.current_board.apply(dir).map(|_| self.path.push(dir))
    }

    pub fn push_step_cloned(&self, dir: Dir) -> Result<Self, ()> {
        let mut board_clone = self.current_board.clone();
        board_clone.apply(dir)?;
        let mut path_clone = self.path.clone();
        path_clone.push(dir);
        Ok(Self {
            current_board: board_clone,
            path: path_clone,
        })
    }
}

#[derive(Clone)]
struct QPath(Path);
impl QPath {
    fn new(p: Path) -> Self {
        Self(p)
    }
    pub fn cost(&self) -> usize {
        let g = self.0.len();
        let f = self.0.current_board.wrong_tiles();
        // let f: usize = self
        //     .0
        //     .current_board()
        //     .inner()
        //     .indexed_iter()
        //     .map(|((x, y), v)| {
        //         let (w, h) = self.0.current_board().size();
        //         let (ox, oy) = if *v == 0 {
        //             (w - 1, h - 1)
        //         } else {
        //             let v = (*v - 1) as usize;
        //             (v % w, v / h)
        //         };
        //         (ox.max(x) - ox.min(x)) + (oy.max(y) - oy.min(y))
        //     })
        //     .sum();
        g + f
    }
}
impl Ord for QPath {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost()).cmp(&self.cost())
    }
}
impl PartialOrd for QPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for QPath {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for QPath {}
