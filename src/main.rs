
mod recipe;
mod recipe_models;


use std::marker::{Send, Sync};
use once_cell::sync::Lazy;
use libp2p::{
    floodsub::{Topic},
    identity::Keypair, 
    PeerId, 
};



const STORAGE_FILE_PATH: &str = "./recipes.json";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static >>;


static KEYS: Lazy<Keypair> = Lazy::new(|| Keypair::generate_ed25519());
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
static TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("recipes"));

fn main() {
    println!("Hello, world!");
}
