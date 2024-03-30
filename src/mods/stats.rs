use std::{fmt::Debug, ops::Add};

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Stats
{
    pub strength: i8,
    pub dexterity: i8,
    pub constitution: i8,
    pub wisdom: i8,
    pub inteligence: i8,
    pub charisma: i8,
}

impl Add for Stats
{
    type Output = Self;
    fn add(self, other: Self) -> Self
    {
        Stats 
        {
            strength: self.strength + other.strength,
            dexterity: self.dexterity + other.dexterity,
            constitution: self.constitution + other.constitution,
            wisdom: self.wisdom + other.wisdom,
            inteligence: self.inteligence + other.inteligence,
            charisma: self.charisma + other.charisma,
        }
    }
}


impl Debug for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stats").field("strength", &self.strength).field("dexterity", &self.dexterity).field("constitution", &self.constitution).field("wisdom", &self.wisdom).field("inteligence", &self.inteligence).field("charisma", &self.charisma).finish()
    }
}

impl Stats
{
    pub fn new() -> Self
    {
        Stats {
            strength: 10,
            dexterity: 10,
            constitution: 10,
            wisdom: 10,
            inteligence: 10,
            charisma: 10,
        }
    }
    pub fn get_raw_ac(&self) -> u8
    {   
        (10 + self.get_dex_mod()).try_into().unwrap()
    }
    pub fn get_str_mod(&self) -> i8
    {
        (self.strength - 10) / 2
    }
    pub fn get_dex_mod(&self) -> i8
    {
        (self.dexterity - 10) / 2
    }
    pub fn get_con_mod(&self) -> i8
    {
        (self.constitution - 10) / 2
    }
    pub fn get_wis_mod(&self) -> i8
    {
        (self.wisdom - 10) / 2
    }
    pub fn get_int_mod(&self) -> i8
    {
        (self.inteligence - 10) / 2
    }
    pub fn get_cha_mod(&self) -> i8
    {
        (self.charisma - 10) / 2
    }
}
