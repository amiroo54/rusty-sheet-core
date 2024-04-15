use core::str;

use crate::mods::{*};

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Class
{
    pub id: Uuid,
    pub name: String,
    pub hit_die: Dice,
    pub actions: Vec<Action>,
    pub subclass_lvl: u8,
    #[serde(skip_deserializing, skip_serializing)]
    pub subclasses: Vec<SubClass>
}


impl Default for Class {
    fn default() -> Self {
        Class
        {
            name: "".to_string(),
            id: Uuid::nil(),
            hit_die: Dice::D4,
            actions: Vec::new(), 
            subclass_lvl: 1,
            subclasses: Vec::new(),
        }
    }
}

impl Class
{
    pub fn new(name: String, hit_die: Dice, subclass_lvl: u8) -> Self
    {
        Class 
        {
            id: Uuid::new_v4(),
            name: name,
            hit_die,
            actions: Vec::new(),
            subclass_lvl,
            subclasses: Vec::new()
        }
    }

}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SubClass
{
    pub class_id: Uuid,
    pub id: Uuid,
    pub name: String,
    pub actions: Vec<Action>
}

impl SubClass {
    pub fn new(name: String, class_id: Uuid) -> Self
    {
        SubClass
        {
            class_id, 
            id: Uuid::new_v4(),
            name,
            actions: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PlayerClass
{
    #[serde(skip_serializing, skip_deserializing)]
    pub class: Class, 
    pub class_id: Uuid,
    pub sub_class_id: Uuid,
    #[serde(skip_serializing, skip_deserializing)]
    pub sub_class: Option<SubClass>,
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
        if let Some(sub_class) = &self.sub_class
        {
            for act in &sub_class.actions
            {
                if act.lvl <= self.lvl
                {
                    actions.push(act);
                }
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
            sub_class_id: Uuid::nil(),
            sub_class: None,
            lvl: 0
        }
    }

    pub fn lvl_up(&mut self) -> Vec<Choice>
    {
        self.lvl += 1;
        let mut choices: Vec<Choice> = Vec::new();
        if self.lvl == self.class.subclass_lvl
        {
            for subclass in &self.class.subclasses
            {
                let choice = Choice
                {
                    description: subclass.name.clone(),
                    effect: |_char| 
                    {
                        
                    },
                };

                choices.push(choice);
            }   
        }

        choices
    }
}