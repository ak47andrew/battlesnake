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
use crate::stuff::datatypes::{CellState, MoveFilter};
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

    let (move_str, value) = moves.iter().max_by_key(|(_, x)| **x).unwrap();
    info!(?move_str, "Chosen move");
    info!(?value, "Move's score");

    Json(
        MoveOutput
        {
            movement: move_str.to_string(),
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