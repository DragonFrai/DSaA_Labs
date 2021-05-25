use ndarray::Array2;
use rand::Rng;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};

const WIDTH: usize = 4;
const HEIGHT: usize = 4;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Board(Array2<u8>);

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct BoardCreateError;

#[inline(always)]
pub fn check_array_size(w: usize, h: usize) -> Result<(), BoardCreateError> {
    if w * h <= 256 {
        Ok(())
    } else {
        Err(BoardCreateError)
    }
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        check_array_size(width, height).expect("Board has more that 255 tiles.");
        let array = Array2::from_shape_fn((width, height), |(x, y)| {
            ((y * width + x + 1) % (width * height)) as u8
        });
        Board(array)
    }

    pub fn from_array(arr: Array2<u8>) -> Result<Self, BoardCreateError> {
        let w = arr.shape()[0];
        let h = arr.shape()[1];
        check_array_size(w, h)?;
        let mut tile_count = vec![0; w * h];
        arr.indexed_iter().for_each(|(_idx, v)| {
            tile_count.get_mut(*v as usize).map(|x| *x += 1);
        });
        let has_one_of_all = tile_count.iter().all(|x| *x == 1);
        if has_one_of_all {
            Ok(Board(arr))
        } else {
            Err(BoardCreateError)
        }
    }

    pub fn size(&self) -> (usize, usize) {
        let w = self.0.shape()[0];
        let h = self.0.shape()[1];
        (w, h)
    }

    pub fn empty_at(&self) -> (usize, usize) {
        self.0
            .indexed_iter()
            .find_map(|(idx, x)| if *x == 0 { Some(idx) } else { None })
            .unwrap()
    }

    pub fn apply(&mut self, dir: Dir) -> Result<(), ()> {
        let (zx, zy) = self.empty_at();
        let (w, h) = self.size();
        match dir {
            Dir::Right if zx < w - 1 => {
                self.0.swap((zx, zy), (zx + 1, zy));
                Ok(())
            }
            Dir::Down if zy < h - 1 => {
                self.0.swap((zx, zy), (zx, zy + 1));
                Ok(())
            }
            Dir::Left if zx > 0 => {
                self.0.swap((zx, zy), (zx - 1, zy));
                Ok(())
            }
            Dir::Up if zy > 0 => {
                self.0.swap((zx, zy), (zx, zy - 1));
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
        let w = self.0.shape()[0];
        let h = self.0.shape()[1];
        self.0
            .indexed_iter()
            .all(|((x, y), v)| ((y * w + x + 1) % (w * h)) as u8 == *v)
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
        self.0
            .indexed_iter()
            .map(|((x, y), v)| {
                if *v == 0 || ((y * w + x + 1) % (w * h)) as u8 == *v {
                    0
                } else {
                    1
                }
            })
            .sum()
    }

    pub fn solve(&self) -> Result<Path, ()> {
        if !self.can_solve() {
            return Err(());
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

        let mut checked_positions = HashSet::new();
        let mut heap = BinaryHeap::with_capacity(1000);

        heap.push(QPath(Path::new(self.clone())));

        let mut i = 0;
        loop {
            i += 1;
            let current = heap.pop().unwrap();
            let x = checked_positions.insert(current.0.current_board.clone());
            if !x {
                continue;
            }
            if i % 100_000 == 0 {
                println!(
                    "iter = {}, steps = {}, euristic = {}, in heap {}",
                    i,
                    current.0.path().len(),
                    current.0.current_board().wrong_tiles(),
                    heap.len()
                );
            }
            // println!("Current board with {}", current.cost());
            if current.0.current_board().is_solved() {
                return Ok(current.0);
            }
            let mut push_or_ignore = |dir| {
                let mut c = current.clone();
                if c.0.push_step(dir).is_ok() && !checked_positions.contains(c.0.current_board()) {
                    heap.push(c);
                }
            }; // 15 2 1 12 8 5 6 11 4 9 10 7 3 14 13 0
            push_or_ignore(Dir::Up);
            push_or_ignore(Dir::Right);
            push_or_ignore(Dir::Down);
            let mut c = current;
            if c.0.push_step(Dir::Left).is_ok() && !checked_positions.contains(c.0.current_board())
            {
                heap.push(c);
            }
        }
    }

    pub fn inner(&self) -> &Array2<u8> {
        &self.0
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = self.0.shape()[0];
        let h = self.0.shape()[1];
        for y in 0..h {
            for x in 0..w {
                match w * h {
                    0..=9 => write!(f, "{:1} ", self.0[(x, y)])?,
                    10..=99 => write!(f, "{:2} ", self.0[(x, y)])?,
                    100..=999 => write!(f, "{:3} ", self.0[(x, y)])?,
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
}
