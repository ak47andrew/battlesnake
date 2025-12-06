use std::{collections::HashMap};

use tracing::info;

use crate::stuff::{datatypes::{Battlesnake, Board, CellState, Coord}, settings, tools};

// pub fn sorted_moves_to_points(
//     available_directions: Vec<&'static str>,
//     head: Coord,
//     dests: &[Coord]
// ) -> &'static str {
//     available_directions
//         .into_iter()
//         .min_by(|a, b| {
//             let dist_a = min_distance(head.shift_by_name(a), dests);
//             let dist_b = min_distance(head.shift_by_name(b), dests);
//             dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
//         })
//         .expect("available_directions is not empty")
// }

// // pub fn get_dangerous_coords(me: Battlesnake, board: Board) -> Vec<Coord> {
//     let mut coords: Vec<Coord> = vec![];

//     for snake in board.snakes {
//         for part in snake.body {
//             coords.push(part);
//             if part == snake.head && snake.id != me.id && snake.length >= me.length {
//                 coords.append(&mut part.surrounded());
//             }
//         }
//     }

//     return coords;
// }

pub fn evaluate(me: &Battlesnake, board: &Board, turn: u32) -> HashMap<&'static str, (f32, CellState)> {
    let directions = ["up", "down", "left", "right"];
    let mut output: HashMap<&'static str, (f32, CellState)> = HashMap::new();
    for dir in directions {
        output.insert(dir, (0.0, CellState::SAFE));
    }

    let head = me.head;
    let threat_board = build_threat_board(&me, &board, turn);
    let lowlevel_heads = get_lowlevel_snakes(me, board);
    
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
            CellState::POTENTIALHEAD => output.insert(dir, (0.0, CellState::POTENTIALHEAD)),
            CellState::POTENTIALTAIL => output.insert(dir, (0.0, CellState::POTENTIALTAIL)),
            CellState::SAFE => {
                // Assigning actual score
                let state: CellState = CellState::SAFE;
                let mut value: f32 = 0.0;

                // Reaching the goal
                let dests: &[Coord];
                if me.health <= 30 || lowlevel_heads.is_empty() {
                    dests = &board.food;
                    info!("Reaching food");
                } else {
                    dests = &lowlevel_heads;
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

pub fn get_lowlevel_snakes(me: &Battlesnake, board: &Board) -> Vec<Coord> {
    let mut output: Vec<Coord> = Vec::new();

    for snake in &board.snakes {
        if snake.id == me.id {
            continue;
        }

        if snake.length <= me.length - 2{
            output.push(snake.head);
        }
    }

    return output;
}

pub fn is_about_to_eat(snake: &Battlesnake, board: &Board) -> bool {
    let head = snake.head;
    for dir in vec!["up", "down", "left", "right"] {
        if board.food.contains(&head.shift_by_name(dir)){
            return true;
        }
    }

    return false;
}

pub fn build_threat_board(me: &Battlesnake, board: &Board, turn: u32) -> Vec<Vec<CellState>> {
    //! IMPORTANT! This is battlesnake's coordinates so buttom left is the (0,0). 
    //! When outputtting be sure to reverse the matrix' first layer so the rows will be in a correct position to see
    //! Tho when doing everything in the battlesnake's coordinates - you'll probably be pretty fine
    let mut threat_board= vec![vec![CellState::SAFE; board.width as usize]; board.height as usize];
    
    for snake in &board.snakes {
        // Head
        if snake.length >= me.length {
            threat_board[snake.head.y as usize][snake.head.x as usize] = CellState::DEATH;
            if snake.id != me.id {
                for s in snake.head.surrounded() {
                    if s.y >= 0 && s.y < board.height && s.x >= 0 && s.x < board.width {
                        threat_board[s.y as usize][s.x as usize] = CellState::POTENTIALHEAD;
                    }
                }
            }
        }
        
        // Body
        for part in &snake.body[1..snake.body.len() - 1] {
            threat_board[part.y as usize][part.x as usize] = CellState::DEATH;
        }

        // Tail
        if !is_about_to_eat(snake, board) && turn >= 3{  // FIXME: When battlesnake ate on the last turn it's tail will be kept in place
            threat_board[snake.body[(snake.length - 1) as usize].y as usize][snake.body[(snake.length - 1) as usize].x as usize] = CellState::SAFE;
        } else {
            threat_board[snake.body[(snake.length - 1) as usize].y as usize][snake.body[(snake.length - 1) as usize].x as usize] = CellState::POTENTIALTAIL;
        }
    }
    
    return threat_board;
}