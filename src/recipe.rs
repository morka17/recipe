use serde::{Serialize, Deserialize};




pub type Recipes = Vec<Recipe>;


#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub id: usize,
    pub name: String,
    pub ingredients: String, 
    pub instructions: String, 
    pub public: bool,
}