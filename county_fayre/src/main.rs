#[derive(Default)]
#[derive(Debug)]
struct Prize {
    value: usize
}

impl Prize {
    fn new(v: usize) -> Prize {
        Prize { value: v }
    }
}

#[derive(Debug)]
struct BoardSquare {
    prize: Option<Prize>
}

impl BoardSquare {
    fn new() -> BoardSquare {
        BoardSquare { prize: None }
    }

    fn contains_prize(&self) -> bool {
        self.prize.is_some()
    }
}

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<BoardSquare>>
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        let mut new_board = Board { grid: Vec::new() };
        for _ in 0..height {
            new_board.grid.push(Vec::new())
        };
        for i in new_board.grid.iter_mut(){
            for _ in 0..width {
                i.push(BoardSquare::new())
            }
        };
        new_board
    }
}

fn main() {
    let mut my_board = Board::new(1, 2);

    let testing_square = &mut my_board.grid[0][0];
    testing_square.prize = Some(Prize::new(1));

    if testing_square.contains_prize() {
        let prize = testing_square.prize.as_ref().unwrap();
        println!("{}", prize.value)
    }
}
