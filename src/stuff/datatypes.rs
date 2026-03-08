use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Game {
    pub id: String,
    pub ruleset: HashMap<String, Value>,
    pub timeout: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Board {
    pub height: i32,
    pub width: i32,
    pub food: Vec<Coord>,
    pub snakes: Vec<Battlesnake>,
    pub hazards: Vec<Coord>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Battlesnake {
    pub id: String,
    pub name: String,
    pub health: i32,
    pub body: Vec<Coord>,
    pub head: Coord,
    pub length: i32,
    pub latency: String,
    pub shout: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn shift_by_name(&self, direction: &str) -> Self {
        match direction {
            "up" => self.shift_by_coord(&Coord { x: 0, y: 1 }),
            "down" => self.shift_by_coord(&Coord { x: 0, y: -1 }),
            "left" => self.shift_by_coord(&Coord { x: -1, y: 0 }),
            "right" => self.shift_by_coord(&Coord { x: 1, y: 0 }),
            _ => *self,
        }
    }

    pub fn shift_by_coord(&self, direction: &Coord) -> Self {
        Coord { x: self.x + direction.x, y: self.y + direction.y }
    }

    #[allow(dead_code, unused)]
    pub fn surrounded(&self) -> Vec<Self> {
        vec![
            self.shift_by_name("up"),
            self.shift_by_name("down"),
            self.shift_by_name("left"),
            self.shift_by_name("right"),
        ]
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameState {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: Battlesnake,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MoveOutput {
    #[serde(rename = "move")]
    pub movement: String,
    pub shout: String,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum CellState {
    SAFE,
    POTENTIAL_HEAD,
    DEATH,
}