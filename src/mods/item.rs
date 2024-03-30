use crate::mods::{*};

use serde::{Serialize, Deserialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Item
{
    pub id: Uuid,
    pub name: String,
    
}

impl FileLoadable for Item
{
    fn get_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn load_from_json(json: &String) -> Self {
        serde_json::from_str(json).unwrap()
    }
}

impl Item
{
    pub fn new(name: String) -> Self
    {
        Item
        {
            id: Uuid::new_v4(),
            name
        }
    }
}
