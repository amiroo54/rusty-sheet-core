use std::{collections::HashMap, vec};

use crate::mods::{*};

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, PartialEq)]
pub struct Character
{
    pub id: Uuid,
    pub name: String,
    
    #[serde(skip_serializing, skip_deserializing)]
    pub race: Race,

    race_id: Uuid,
    
    pub classes: Vec<PlayerClass>,
        
    #[serde(skip_serializing, skip_deserializing)]
    pub inventory: Vec<Item>,
    
    item_ids: Vec<Uuid>,

    pub hp: i32,
    pub ac: u8,

    pub base_stat: Stats,
    actions: Vec<Action>,
}


impl Character {
    pub fn new(name: String, race: Race, starting_class: Class, stats: Stats) -> Self
    {
        let mut classes: Vec<PlayerClass> = Vec::new();
        classes.push(PlayerClass::new(starting_class));
        let ac = stats.get_raw_ac();
        Character {
            id: Uuid::new_v4(),
            name, 
            race_id: race.id.clone(),
            race, 
            classes,
            inventory: Vec::new(), 
            item_ids: Vec::new(),
            hp: 0,
            ac,
            base_stat: stats,
            actions: Vec::new()}
    }

    pub fn populate_from_ids(&mut self, class_list: &HashMap<Uuid ,Class>, race_list: &HashMap<Uuid, Race>, item_list: &HashMap<Uuid, Item>)
    {
        let mut classes: Vec<PlayerClass> = Vec::new();
        for class in &self.classes
        {
            classes.push(PlayerClass::new(class_list[&class.class_id].clone()));
        }
        self.classes = classes;

        let race = race_list[&self.race_id].clone();
        self.race = race;

        let mut items: Vec<Item> = Vec::new();
        for item_id in &self.item_ids
        {
            items.push(item_list[item_id].clone());
        }
        self.inventory = items;
    }

    pub fn get_possible_actions(&self) -> Vec<&Action>
    {
        let mut actions: Vec<&Action> = vec![];
        actions.extend(self.race.get_actions());
        for class in &self.classes
        {
            actions.extend(class.get_actions());
        }
        actions
    }
    pub fn get_max_hp(&self) -> i32
    {
        todo!()
    }

}

impl Default for Character {
    fn default() -> Self {
        Character::new(String::from(""), Race::new("".to_string()), Class::new("".to_string(), Dice::D10), Stats::new())
    }
}