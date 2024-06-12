use crate::mods::{*};

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Race
{
    pub id: Uuid,
    pub name: String,
    pub decription: String,
    pub actions: Vec<Action>
}

impl Actionable for Race {
    fn get_actions(&self) -> Vec<&Action> {
        self.actions.iter().collect()
    }
}

impl Default for Race {
    fn default() -> Self {
        Self { id: Default::default(), name: Default::default(), decription: Default::default(), actions: Default::default() }
    }
}


impl Race
{
    pub fn new(name: String) -> Self
    {
        Race {
            id: Uuid::new_v4(),
            name,
            decription: "".to_string(),
            actions: Vec::new()
        }
    }
}