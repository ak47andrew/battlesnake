mod stuff;

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use rand::prelude::IndexedRandom;

use actix_web::web::Json;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

use stuff::datatypes::{GameState, MoveOutput};
use tracing::{info, instrument};

use crate::stuff::algorithms;
use crate::stuff::datatypes::CellState;
use crate::stuff::tools::{get_app_mode, AppMode};

// TODO: space check: just ln (x / n), don't overcomplicate things

#[get("/")]
async fn index() -> impl Responder {
    match get_app_mode() {
        AppMode::DEV => {
            Json(serde_json::json!(
                {
                    "apiversion": "1",
                    "author": "ak47andrew",
                    "color": "#7d7d7d",
                    "head": "safe",
                    "tail": "freckled"
                })
            )
        }
        AppMode::PROD => {
            Json(serde_json::json!(
                {
                    "apiversion": "1",
                    "author": "ak47andrew",
                    "color": "#FF0000",
                    "head": "crystal-power",
                    "tail": "nr-booster"
                })
            )
        }
    }
}

#[post("/start")]
async fn handle_start(start_req: Json<GameState>) -> impl Responder {
    println!("Started game at https://play.battlesnake.com/game/{} Snakes:", start_req.game.id);
    for snake in start_req.board.snakes.iter() {
        println!("- {} ({})", snake.name, snake.id);
    }
    HttpResponse::Ok().finish()
}

#[post("/move")]
#[instrument(skip(move_req))]
async fn handle_move(move_req: Json<GameState>) -> Json<MoveOutput> {
    info!(?move_req, "Move request received");

    let moves = algorithms::evaluate(&move_req.you, &move_req.board);
    info!(?moves, "Available moves");

    for state in vec![CellState::SAFE, CellState::POTENTIAL_HEAD] {
        let sub_moves = moves
                        .iter()
                        .filter(|x: &(&&str, &(f32, CellState))| x.1.1 == state)
                        .map(|(k, v)| (*k, *v))
                        .collect::<HashMap<&str, (f32, CellState)>>();
        
        if sub_moves.is_empty() {
            info!("No moves found for state {:?}", state);
            continue;
        }
        info!(?sub_moves, "Moves for state {:?}", state);
        
        let max_value = sub_moves.values().max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)).unwrap().0;
        info!(?sub_moves, "Best value for state {:?}: {:?}", state, max_value);
    
        let max_moves: Vec<&str> = sub_moves.iter()
                                                .filter(|(_, v)| v.0 == max_value)
                                                .map(|(k, _)| *k)
                                                .collect();
        
        info!("There's {} moves to be chosen", max_moves.len());
        
        let random_move = max_moves.choose(&mut rand::rng()).unwrap();
        info!(?random_move, "Chosen move");
        
        return Json(
            MoveOutput 
            { 
                movement: random_move.to_string(),
                shout: String::from("AAAAAAAAAAAAAAAAAAAAAAAA")
            }
        )
    }

    Json(
        MoveOutput 
        { 
            movement: vec!["up", "down", "left", "right"].choose(&mut rand::rng()).unwrap().to_string(),
            shout: String::from("AAAAAAAAAAAAAAAAAAAAAAAA") 
        }
    )
}

#[post("/end")]
async fn handle_end(end_req: Json<GameState>) -> impl Responder {
    println!("Finished game at https://play.battlesnake.com/game/{} in {} moves", end_req.game.id, end_req.turn);

    if end_req.board.snakes.len() != 0 {
        if end_req.you.id == end_req.board.snakes[0].id {
            println!("I won");
        } else {
            println!("I lost, {} won", end_req.board.snakes[0].name);
        }
    } else {
        println!("Everyone lost/tie");
    }

    HttpResponse::Ok().finish()
}

fn setup_logging() {
    tracing_subscriber::fmt()
                        .with_writer(File::create(Path::new("output.log")).unwrap())
                        .json()
                        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logging();
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(handle_start)
            .service(handle_move)
            .service(handle_end)
    })
    .bind(("0.0.0.0", 9100))?
    .run()
    .await
}