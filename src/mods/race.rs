
use crate::mods::{*};

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Race
{
    pub id: Uuid,
    pub name: String,
    pub actions: Vec<Action>
}

impl Actionable for Race {
    fn get_actions(&self) -> Vec<&Action> {
        let mut actions: Vec<&Action> = Vec::new();
        for act in &self.actions
        {
            actions.push(&act);
        }
        actions
    }
}

impl Default for Race {
    fn default() -> Self {
        Self { id: Default::default(), name: Default::default(), actions: Default::default() }
    }
}
impl Default for &Race {
    fn default() -> Self {
        todo!()
    }
}

impl Race
{
    pub fn new(name: String) -> Self
    {
        Race {
            id: Uuid::new_v4(),
            name,
            actions: Vec::new()
        }
    }
}