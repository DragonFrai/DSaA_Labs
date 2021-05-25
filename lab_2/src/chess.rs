

#[derive(Copy, Clone)]
pub struct Figure
{
    pub x: i32,
    pub y: i32,
}

impl Figure {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct Chess {
    board: Box<[[bool; 8]; 8]>,
}

impl Chess {
    pub fn new() -> Self {
        Self {
            board: Box::new([[true; 8]; 8]),
        }
    }

    pub fn reset(&mut self) {
        for line in &mut self.board.iter_mut() {
            for x in line {
                *x = true;
            }
        }
    }

    pub fn remove_position(&mut self, mut i: usize, mut j: usize) {
        for k in 0..8 {
            self.board[i][k] = false;
            self.board[k][j] = false;
        }

        let mut temp_i = i;
        let mut temp_j = j;

        temp_i -= usize::min(temp_i, temp_j);
        temp_j -= usize::min(temp_i, temp_j);

        while temp_i != 7 && temp_j != 7 {
            self.board[temp_i][temp_j] = false;
            temp_i += 1;
            temp_j += 1;
        }

        while i!=7 && j!=0 {
            i += 1;
            j -= 1;
        }

        while i!=0 && j!=7
        {
            self.board[i][j]=false;
            i -= 1;
            j += 1;
        }
    }

    pub fn check(temp: Figure, table: &[Figure]) -> bool {
        for i in table {
            if i.x == temp.x && i.y == temp.y {
                return true;
            }
        }
        false
    }

    pub fn solve(&mut self) -> Vec<Figure> {
        let mut places = Vec::new();
        let mut i = 0i32;
        let mut j = 0;
        while places.len()!=8 {
            if self.board[i as usize][j as usize] {
                let place = Figure::new(j, i);
                places.push(place);
                self.remove_position(i as usize, j as usize);
                i += 2;
                j += 1;
            }
            if i>7
            {
                i = 0;
            }

            while j < 8 && !self.board[i as usize][j as usize]
            {
                i = (i + 1) % 8;
            }
        }
        return places;
    }

    pub fn printBoard(&self, table: &[Figure]) {
        let mut quieen = Figure::new(0, 0);

        for i in 0..8 {
            for j in 0..8 {
                quieen.x = j;
                quieen.y = i;

                if Self::check(quieen, table) {
                    print!("Q ");
                } else if self.board[i as usize][j as usize] {
                    print!("+ ");
                } else {
                    print!("- ");
                }
            }
            println!();
        }
    }
}

