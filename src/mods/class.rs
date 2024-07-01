use core::str;
use std::collections::HashMap;

use crate::mods::*;

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
    pub subclasses: HashMap<Uuid, SubClass>
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
            subclasses: HashMap::new(),
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
            subclasses: HashMap::new()
        }
    }

    pub fn create_subclass(&mut self, name: String) -> Uuid
    {
        let id = Uuid::new_v4();
        let s = SubClass
        {
            class_id: self.id.clone(),
            id: id.clone(),
            name,
            actions: Vec::new(),
        };
        self.subclasses.insert(s.id.clone(), s);
        id
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
        let mut choices = vec![];
        if self.lvl == self.class.subclass_lvl
        {
            let mut subclass_choice: Choice = Choice::new("Choose your subclass".to_string());
            for subclass in self.class.subclasses.clone()
            {
                let binding = subclass.clone();
                let id = self.class_id.clone();
                let effect = move |char: &mut Character| 
                {
                    let class = char.get_class(&id).unwrap();
                    class.sub_class_id = binding.0.clone();
                    class.sub_class = Some(binding.1);
                    
                };
                let option = ChoiceOption
                {
                    description: subclass.1.name.clone(),
                    effect: Box::new(effect)
                };
                subclass_choice.options.push(option);
            }   
            choices.push(subclass_choice);
        }

        choices
    }
}