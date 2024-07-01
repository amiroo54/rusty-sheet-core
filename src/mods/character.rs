use std::{collections::HashMap, vec};

use crate::{mods::*, Data};

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, PartialEq)]
pub struct Character
{
    pub id: Uuid,
    pub name: String,
    
    #[serde(skip_serializing, skip_deserializing)]
    pub race: Option<Race>,

    pub race_id: Uuid,
    
    pub classes: Vec<PlayerClass>,
        
    #[serde(skip_serializing, skip_deserializing)]
    pub inventory: Vec<Item>,
    
    pub item_ids: Vec<Uuid>,

    pub hp: i32,
    pub ac: u8,

    pub base_stat: Stats,
    actions: Vec<Action>,
}


impl Character {
    pub fn new(name: String) -> Self
    {
        Character {
            id: Uuid::new_v4(),
            name, 
            race_id: Uuid::nil(),
            race: None, 
            classes: vec![],
            inventory: Vec::new(), 
            item_ids: Vec::new(),
            hp: 0,
            ac: 0,
            base_stat: Stats::new(),
            actions: Vec::new()}
    }

    pub fn populate_from_ids(&mut self, data: &Data)
    {
        let mut classes: Vec<PlayerClass> = Vec::new();
        for class in &self.classes
        {
            classes.push(PlayerClass::new(data.class_list[&class.class_id].clone()));
        }
        self.classes = classes;
        let race = data.race_list[&self.race_id].clone();
        self.race = Some(race);

        let mut items: Vec<Item> = Vec::new();
        for item_id in &self.item_ids
        {
            items.push(data.item_list[item_id].clone());
        }
        self.inventory = items;
    }

    pub fn get_possible_actions(&self) -> Vec<&Action>
    {
        let mut actions: Vec<&Action> = vec![];
        actions.extend(&self.race.as_ref().unwrap().get_actions());
        for class in &self.classes
        {
            actions.extend(class.get_actions().clone());
        }
        actions
    }
    pub fn get_max_hp(&self) -> i32
    {
        todo!()
    }
    pub fn get_class(&mut self, id: &Uuid) -> Option<&mut PlayerClass>
    {
        for class in &mut self.classes
        {
            if class.class_id == id.clone()
            {
                return Some(class);
            }
        }
        None
    }
}

impl Default for Character {
    fn default() -> Self {
        Character::new(String::from(""))
    }
}