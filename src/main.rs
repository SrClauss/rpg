#![allow(dead_code, unused_imports)]

mod db;
mod dice;
mod llm;
mod map;
pub mod rules;

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("============================================================");
    println!("          MOTOR RPG HEXAGONAL TÁTICO - RUST LOGIC           ");
    println!("============================================================\n");

    println!("Motor de Regras pronto e operante!");

    Ok(())
}
