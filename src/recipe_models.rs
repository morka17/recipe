
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
enum ListMode {
    ALL,
    One(String), 
}


#[derive(Serialize, Deserialize, Debug)]
struct ListRequest{
    mode: ListMode,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListResponse {
    mode: ListMode,
    data: Recipes,
}