use crate::stuff::datatypes::{Battlesnake, Board, CellState, Coord};
use crate::stuff::settings::{FOOD_APPROACH_MODIFIER, HEAD_APPROACH_MODIFIER};
use crate::stuff::tools::min_distance;

pub const EVAL_ALGOS: [fn(&EvalData, CellState) -> CellState; 2] = [
    food_dest,
    aggressive_dest,
    // space_check
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

    let dist = min_distance(*data.next_head, &*data.board.food) as f32;
    let a = FOOD_APPROACH_MODIFIER * (100.4 - data.me.health as f32);
    let area = (data.board.width + data.board.height) as f32;
    state.add_value(a / dist - a / area)
}

pub fn aggressive_dest(data: &EvalData, state: CellState) -> CellState {
    if state == CellState::DEATH {
        return state;
    }

    let low_level_heads = get_low_level_snakes(data.me, data.board);
    if low_level_heads.is_empty() {
        return state;
    }

    let dist = min_distance(*data.next_head, &*low_level_heads) as f32;
    let b = HEAD_APPROACH_MODIFIER * dist;
    let area = (data.board.width + data.board.height) as f32;
    state.add_value(b / dist - b / area)
}

pub fn space_check(data: &EvalData, state: CellState) -> CellState {
    if state == CellState::DEATH {
        return state;
    }

    state // TODO
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
