use std::collections::VecDeque;
use crate::stuff::algorithms::build_threat_board;
use crate::stuff::datatypes::{Battlesnake, Board, CellState, Coord};
use crate::stuff::settings::{FOOD_APPROACH_MODIFIER, HEAD_APPROACH_MODIFIER, SPACE_HIGH_BASE, SPACE_LOW_BASE};
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
    let b = HEAD_APPROACH_MODIFIER * data.me.health as f32;
    let area = (data.board.width + data.board.height) as f32;
    state.add_value(b / dist - b / area)
}

pub fn space_check(data: &EvalData, state: CellState) -> CellState {
    if state == CellState::DEATH {
        return state;
    }

    let thread_board = build_threat_board(&data.me, &data.board);
    let space = count_space(data.next_head, &thread_board, &data.board) as f32;
    state.add_value((space / data.me.length as f32).log(
        if space <= data.me.length as f32 {SPACE_LOW_BASE} else {SPACE_HIGH_BASE}
    ))
}

pub fn count_space(start: &Coord, thread_board: &Vec<Vec<CellState>>, board: &Board) -> u32 {
    let mut visited = vec![vec![false; board.width as usize]; board.height as usize];
    let mut queue = VecDeque::new();

    queue.push_back(*start);

    let mut count = 0;

    while let Some(p) = queue.pop_front() {
        if !thread_board[p.y as usize][p.x as usize].is_safe(false) {
            continue;
        }
        if visited[p.y as usize][p.x as usize] {
            continue;
        }

        count += 1;
        visited[p.y as usize][p.x as usize] = true;

        for pp in p.surrounded() {
            if pp.x < 0 || pp.y < 0 || pp.x >= board.width || pp.y >= board.height {
                continue;
            }

            queue.push_back(pp);
        }
    }

    count
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
