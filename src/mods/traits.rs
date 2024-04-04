use crate::mods::{*};

pub trait Actionable
{
    fn get_actions(&self) -> Vec<&Action>;
}  

pub trait Stateable
{
    fn get_added_stats(&self) -> Stats;
}

pub trait Choicable
{
    fn get_possible_choices(&self, character: &Character) -> Vec<Choice>;
}

pub struct Choice
{
    pub description: String, 
    pub effect: fn(&mut Character)
}