use crate::stuff::datatypes::{Battlesnake, Board, CellState, Coord};
use crate::stuff::settings::{FOOD_APPROACH_MODIFIER, HEAD_APPROACH_MODIFIER};
use crate::stuff::tools::min_distance;

pub const EVAL_ALGOS: [fn(&EvalData, CellState) -> CellState; 3] = [
    food_dest,
    aggressive_dest,
    space_check
];

pub struct EvalData<'a> {
    pub me: &'a Battlesnake,
    pub board: &'a Board,
    pub next_head: &'a Coord
}

pub fn food_dest(data: &EvalData, state: CellState) -> CellState {
    if state == CellState::DEATH {
        return state;
    }

    let dist = min_distance(*data.next_head, &*data.board.food);
    state.add_value(FOOD_APPROACH_MODIFIER / dist as f32 - FOOD_APPROACH_MODIFIER / 100.0)
}

pub fn aggressive_dest(data: &EvalData, state: CellState) -> CellState {
    if state == CellState::DEATH {
        return state;
    }

    let low_level_heads = get_low_level_snakes(data.me, data.board);
    if low_level_heads.is_empty() {
        return state;
    }

    let dist = min_distance(*data.next_head, &*low_level_heads);
    state.add_value(HEAD_APPROACH_MODIFIER / dist as f32 - HEAD_APPROACH_MODIFIER / (data.board.width + data.board.height) as f32)
}

pub fn space_check(data: &EvalData, state: CellState) -> CellState {
    if state == CellState::DEATH {
        return state;
    }

    todo!()
}

pub fn get_low_level_snakes(me: &Battlesnake, board: &Board) -> Vec<Coord> {
    let mut output: Vec<Coord> = Vec::new();

    for snake in &board.snakes {
        if snake.id == me.id {
            continue;
        }

        if snake.length < me.length {
            output.push(snake.head);
        }
    }

    output
}

// // Reaching the goal
// let dests: &[Coord];
// if me.health <= 30 || low_level_heads.is_empty() {  // \left\{x<30:\ 1,\ x\ge30:\ \frac{\left(-x+100\right)}{70}\right\}\left\{x\ge0\right\}\left\{x\le100\right\}
// dests = &board.food;
// info!("Reaching food");
// } else {
// dests = &low_level_heads;
// info!("Reaching low-level snakes");
// }
//
// let dist = tools::min_distance(next_head, dests);
// let state = state.add_value(settings::APPROACH_MODIFIER / dist as f32);