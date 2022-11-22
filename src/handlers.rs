use tokio::{fs};
use log::{info, error};
use libp2p::{swarm::Swarm};

use std::collections::HashSet;

use crate::recipe_behaviour::RecipeBehaviour;
use crate::recipe::{Recipes, Recipe};
use crate::recipe_models::{ListRequest, ListMode};
use crate::recipe_behaviour::TOPIC;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

const STORAGE_FILE_PATH: &str = "./recipes.json";



pub async fn handle_list_peers(swarm: &mut Swarm<RecipeBehaviour>){
    info!("Discovered Peers:");
    let nodes = swarm.behaviour().mdns.discovered_nodes();
    let mut unique_peers = HashSet::new();
    for peer in nodes{
        unique_peers.insert(peer);
    }
    unique_peers.iter().for_each(|p| info!("{}", p));
}

pub async fn handle_list_recipes(cmd: &str, swarm: &mut Swarm<RecipeBehaviour>) {
    let rest = cmd.strip_prefix("ls r ");
    match rest {
        Some("all") => {
            let req = ListRequest {
                mode: ListMode::All,
            };
            let json = serde_json::to_string(&req).expect("can jsonify request");
            swarm
                .behaviour_mut()
                .floodsub
                .publish(TOPIC.clone(), json.as_bytes());
        }
        Some(recipes_peer_id) => {
            let req = ListRequest {
                mode: ListMode::One(recipes_peer_id.to_owned()),
            };
            let json = serde_json::to_string(&req).expect("can jsonify request");
            swarm
                .behaviour_mut()
                .floodsub
                .publish(TOPIC.clone(), json.as_bytes());
        }
        None => {
            match read_local_recipes().await {
                Ok(v) => {
                    info!("Local Recipes ({})", v.len());
                    v.iter().for_each(|r| info!("{:?}", r));
                }
                Err(e) => error!("error fetching local recipes: {}", e),
            };
        }
    };
}


pub async fn handle_create_recipe(cmd: &str) {
    if let Some(rest) = cmd.strip_prefix("create r") {
        let elements: Vec<&str> = rest.split("|").collect();
        if elements.len() < 3 {
            info!("too few arguments - Format: name|ingredients|instructions|public(bool) ");

        }else {
            let name = elements[0];
            let ingredients= elements[1];
            let instructions = elements[2];
            let public: bool = match elements.get(3){
                Some(e) =>  {
                    match *e {
                        "true" => true,
                        "false" => false,
                        "t" => true,
                        "f" => false,
                        _  => false, 
                    }
                },
                None => false,
            };

            if let Err(e) = create_new_recipe(name, ingredients, instructions, public).await{
                error!("error creating recipe: {}", e);
            }
        }
    }
}


pub async fn handle_publish_recipe(cmd: &str) {
    if let Some(rest) = cmd.strip_prefix("publish r") {
        match rest.trim().parse::<usize>(){
            Ok(id) => {
                if let Err(e) = publish_recipe(id).await {
                    info!("error publishing recipe with id {}, {}", id, e);
                }else {
                    info!("Published Recipe with id: {}", id);
                }
            },
            Err(e) => error!("Invalid id: {}, {}", rest.trim(), e),
        }
    }
}

async fn create_new_recipe(name: &str, ingredients: &str, instructions: &str, public: bool)  -> Result<()> {
    let mut local_recipes = read_local_recipes().await?;
    let new_id = match local_recipes.iter().max_by_key(|r| r.id) {
        Some(v) => v.id + 1,
        None => 0,
    };

    local_recipes.push(Recipe {
        id: new_id,
        name: name.to_owned(),
        ingredients: ingredients.to_owned(),
        instructions: instructions.to_owned(),
        public: public,
    });

    write_local_recipes(&local_recipes).await?;

    info!("created recipe");
    info!("Name: {}", name);
    info!("Ingredients: {}", ingredients);
    info!("Instructions: {}", instructions);

    Ok(())
}


async fn publish_recipe(id: usize) -> Result<()> {
    let mut local_recipes = read_local_recipes().await?;
    local_recipes
    .iter_mut()
    .filter(|r| r.id == id)
    .for_each(|r| r.public = true);

    write_local_recipes(&local_recipes).await?;
    Ok(())
}


pub async fn read_local_recipes() -> Result<Recipes> {
    let content = fs::read(STORAGE_FILE_PATH).await?;
    let result = serde_json::from_slice(&content)?;
    Ok(result)
}

async fn write_local_recipes(recipes: &Recipes) -> Result<()> {
    let json = serde_json::to_string(&recipes)?;
    fs::write(STORAGE_FILE_PATH, &json).await?;
    Ok(())
} 