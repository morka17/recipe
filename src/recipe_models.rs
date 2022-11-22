use crate::recipe::Recipes;


use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum ListMode {
    All,
    One(String), 
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ListRequest{
    pub mode: ListMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListResponse {
    pub mode: ListMode,
    pub data: Recipes,
    pub receiver: String,
}

pub enum EventType {
    Response(ListResponse),
    Input(String)
}