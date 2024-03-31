use crate::mods::{*};

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Class
{
    pub id: Uuid,
    pub name: String,
    pub hit_die: Dice,
    pub actions: Vec<Action>
}


impl Default for Class {
    fn default() -> Self {
        Class
        {
            name: "".to_string(),
            id: Uuid::nil(),
            hit_die: Dice::D4,
            actions: Vec::new()
        }
    }
}

impl Class
{
    pub fn new(name: String, dice: Dice) -> Self
    {
        Class 
        {
            id: Uuid::new_v4(),
            name: name,
            hit_die: dice,
            actions: Vec::new(),
        }
    }

}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PlayerClass
{
    pub class_id: Uuid,
    #[serde(skip_serializing, skip_deserializing)]
    pub class: Class, 
    pub lvl: u8,
}

impl Actionable for PlayerClass
{
    fn get_actions(&self) -> Vec<&Action>
    {
        let mut actions: Vec<&Action> = Vec::new();
        for act in &self.class.actions
        {
            if act.lvl <= self.lvl
            {
                actions.push(&act);
            }
        }
        actions
    }
}

impl PlayerClass {
    pub fn new(class: Class) -> Self
    {
        PlayerClass
        {
            class_id: class.id.clone(),
            class, 
            lvl: 1
        }
    }
}