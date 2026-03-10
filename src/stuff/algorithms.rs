use std::{collections::HashMap};

use crate::stuff::datatypes::{Battlesnake, Board, CellState};
use crate::stuff::move_algos::{EvalData, EVAL_ALGOS};

pub fn evaluate(me: &Battlesnake, board: &Board) -> HashMap<&'static str, CellState> {
    let directions = ["up", "down", "left", "right"];
    let mut output: HashMap<&'static str, CellState> = HashMap::new();
    for dir in directions {
        output.insert(dir, CellState::SAFE(0.0));
    }

    let head = me.head;
    let threat_board = build_threat_board(&me, &board);

    // // Step 1. Check boundaries
    if head.x == 0 {
        output.insert("left", CellState::DEATH);
    } else if head.x == board.width - 1 {
        output.insert("right", CellState::DEATH);
    }

    if head.y == 0 {
        output.insert("down", CellState::DEATH);
    } else if head.y == board.height - 1 {
        output.insert("up", CellState::DEATH);
    }

    // Finally evaluation function!
    for &dir in directions.iter() {
        if CellState::DEATH == output[dir] {
            continue;
        }
        let next_head = head.shift_by_name(&dir);
        match threat_board[next_head.y as usize][next_head.x as usize] {
            CellState::DEATH => output.insert(dir, CellState::DEATH),
            init_state => {
                // Assigning actual score
                let mut state: CellState = init_state;
                let eval_data = EvalData {
                    me,
                    board,
                    next_head: &next_head
                };

                for step in EVAL_ALGOS {
                    state = step(&eval_data, init_state);
                }

                output.insert(dir, state)
            }
        };
    }

    output
}

pub fn build_threat_board(me: &Battlesnake, board: &Board) -> Vec<Vec<CellState>> {
    let mut threat_board= vec![vec![CellState::SAFE (0.0); board.width as usize]; board.height as usize];
    
    for snake in &board.snakes {
        // Head
        if snake.length >= me.length {
            threat_board[snake.head.y as usize][snake.head.x as usize] = CellState::DEATH;
            if snake.id != me.id {
                for s in snake.head.surrounded() {
                    if s.y >= 0 && s.y < board.height && s.x >= 0 && s.x < board.width {
                        threat_board[s.y as usize][s.x as usize] = CellState::POTENTIAL_HEAD (0.0);
                    }
                }
            }
        }
        
        // Body
        for part in &snake.body[1..snake.body.len() - 1] {
            threat_board[part.y as usize][part.x as usize] = CellState::DEATH;
        }
    }
    
    threat_board
}