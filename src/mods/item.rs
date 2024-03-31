use crate::mods::{*};

use serde::{Serialize, Deserialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Item
{
    pub id: Uuid,
    pub name: String,
    actions: Vec<Action>,
}

impl Actionable for Item {
    fn get_actions(&self) -> Vec<&Action> {
        self.actions.iter().collect()
    }
}

impl Item
{
    pub fn new(name: String) -> Self
    {
        Item
        {
            id: Uuid::new_v4(),
            name,
            actions: Vec::new()
        }
    }
}
