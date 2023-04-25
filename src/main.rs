use std::{thread, time::Duration};

use game_of_life::Board;

fn main() {
    let mut board = Board{width: 60, height: 60, state:vec![vec![0; 60]; 60]};
    board.random_state();
    let mut iteration = 1;
    loop{
        iteration+=1;
        board.render();
        println!("Generation {}", iteration);
        println!("---------------------");
        board.next_board_state();
        thread::sleep(Duration::from_millis(100));
    }
}
