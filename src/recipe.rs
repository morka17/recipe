use serde::{Serialize, Deserialize};




pub type Recipes = Vec<Recipe>;


#[derive(Serialize, Deserialize, Debug)]
struct Recipe {
    id: usize,
    name: String,
    ingredients: String, 
    instructions: String, 
    public: bool,
}