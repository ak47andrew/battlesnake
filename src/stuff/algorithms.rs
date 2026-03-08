use std::{collections::HashMap};

use tracing::info;

use crate::stuff::{datatypes::{Battlesnake, Board, CellState, Coord}, settings, tools};

pub fn evaluate(me: &Battlesnake, board: &Board) -> HashMap<&'static str, (f32, CellState)> {
    let directions = ["up", "down", "left", "right"];
    let mut output: HashMap<&'static str, (f32, CellState)> = HashMap::new();
    for dir in directions {
        output.insert(dir, (0.0, CellState::SAFE));
    }

    let head = me.head;
    let threat_board = build_threat_board(&me, &board);
    let low_level_heads = get_low_level_snakes(me, board);
    
    // // Step 1. Check boundaries
    if head.x == 0 {
        output.insert("left", (f32::NEG_INFINITY, CellState::DEATH));
    } else if head.x == board.width - 1 {
        output.insert("right", (f32::NEG_INFINITY, CellState::DEATH));
    }

    if head.y == 0 {
        output.insert("down", (f32::NEG_INFINITY, CellState::DEATH));
    } else if head.y == board.height - 1 {
        output.insert("up", (f32::NEG_INFINITY, CellState::DEATH));
    }

    // Finally evaluation function!
    for &dir in directions.iter() {
        if output[dir].0 == f32::NEG_INFINITY {
            continue;
        }
        let next_head = head.shift_by_name(&dir);
        match threat_board[next_head.y as usize][next_head.x as usize] {
            CellState::DEATH => output.insert(dir, (f32::NEG_INFINITY, CellState::DEATH)),
            CellState::POTENTIAL_HEAD => output.insert(dir, (0.0, CellState::POTENTIAL_HEAD)),
            CellState::SAFE => {
                // Assigning actual score
                let state: CellState = CellState::SAFE;
                let mut value: f32 = 0.0;

                // Reaching the goal
                let dests: &[Coord];
                if me.health <= 30 || low_level_heads.is_empty() {
                    dests = &board.food;
                    info!("Reaching food");
                } else {
                    dests = &low_level_heads;
                    info!("Reaching low-level snakes");
                }

                let dist = tools::min_distance(next_head, dests);
                value += settings::APPROACH_MODIFIER / dist as f32;

                // TODO: add flood fill for safe areas and stuff. Make it a priority over the approach

                output.insert(dir, (value, state))
            },
        };
    }


    output
}

pub fn get_low_level_snakes(me: &Battlesnake, board: &Board) -> Vec<Coord> {
    let mut output: Vec<Coord> = Vec::new();

    for snake in &board.snakes {
        if snake.id == me.id {
            continue;
        }

        if snake.length <= me.length - 2 {
            output.push(snake.head);
        }
    }

    output
}

pub fn build_threat_board(me: &Battlesnake, board: &Board) -> Vec<Vec<CellState>> {
    let mut threat_board= vec![vec![CellState::SAFE; board.width as usize]; board.height as usize];
    
    for snake in &board.snakes {
        // Head
        if snake.length >= me.length {
            threat_board[snake.head.y as usize][snake.head.x as usize] = CellState::DEATH;
            if snake.id != me.id {
                for s in snake.head.surrounded() {
                    if s.y >= 0 && s.y < board.height && s.x >= 0 && s.x < board.width {
                        threat_board[s.y as usize][s.x as usize] = CellState::POTENTIAL_HEAD;
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