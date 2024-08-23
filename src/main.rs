mod domain;

use crate::domain::*;
use std::process::Command;

fn print_board(board: &Board) {
    println!(" {} | {} | {}", board[0], board[1], board[2]);
    println!("-----------");
    println!(" {} | {} | {}", board[3], board[4], board[5]);
    println!("-----------");
    println!(" {} | {} | {}", board[6], board[7], board[8]);

}

fn mark_player(board: &mut Board, position: usize, current_turn: &Player) -> Option<&'static str> {
    if let Some(cell) = board.get(position) {
        match cell {
            Cell::Coordinate(coordinate) => {
                match *current_turn {
                    Player::X => board[position] = Cell::XMark,
                    Player::O => board[position] = Cell::OMark,
                }

                return None;
            },
            _ => return Some("The position was occupied"),
        }
    } else {
        return Some("The position is not exists");
    }
}

fn check_winner(board: &Board) -> Option<Player> {
    let mut cell: Option<Cell> = None;

    if board[0] == board[1] && board[1] == board[2] && board[0] == board[2] {
        cell = Some(board[0]);
    } else if board[3] == board[4] && board[4] == board[5] && board[3] == board[5] {
        cell = Some(board[3]);
    } else if board[6] == board[7] && board[7] == board[8] && board[6] == board[8] {
        cell = Some(board[6]);
    } else if board[0] == board[3] && board[3] == board[6] && board[0] == board[6] {
        cell = Some(board[0]);
    } else if board[1] == board[4] && board[4] == board[7] && board[1] == board[7] {
        cell = Some(board[1]);
    } else if board[2] == board[5] && board[5] == board[8] && board[2] == board[8] {
        cell = Some(board[2]);
    } else if board[0] == board[4] && board[4] == board[8] && board[0] == board[8] {
        cell = Some(board[0]);
    } else if board[2] == board[4] && board[4] == board[6] && board[2] == board[6] {
        cell = Some(board[2]);
    }

    match cell {
        Some(value) => {
            match value {
                Cell::XMark => return Some(Player::X),
                Cell::OMark => return Some(Player::O),
                _ => panic!("Something wrong !!!"),
            }
        },
        None => return None,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut current_turn = Player::X;
    let mut board: Board = [
        Cell::Coordinate(0), Cell::Coordinate(1), Cell::Coordinate(2),
        Cell::Coordinate(3), Cell::Coordinate(4), Cell::Coordinate(5),
        Cell::Coordinate(6), Cell::Coordinate(7), Cell::Coordinate(8),
    ];
    let mut running = true;

    loop {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "cls"])
                .status()?;
        } else {
            Command::new("clear").status()?;
        }

        print_board(&board);

        println!("Turn player {:?}", current_turn);
        println!("What is yout position to mark ? ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error to read line");
        
        let position = input.trim().parse::<usize>().expect("Error to parse input to usize");

        match mark_player(&mut board, position, &current_turn) {
            None => current_turn = if current_turn == Player::X { Player::O } else { Player::X },
            Some(message) => {
                println!("{}", message);
                continue;
            }
        }

        match check_winner(&board) {
            Some(player) => {
                match player {
                    Player::X => println!("Player X are the winner"),
                    Player::O => println!("Player O are the winner"),
                }

                running = false;
            },
            None => {
                if board.iter().all(|cell| *cell != Cell::XMark || *cell != Cell::OMark) {
                    println!("The game is draw");
                }
            },
        }


        if !running {
            break;
        }
    }

    println!("Game result: ");
    print_board(&board);
    
    Ok(())
}

