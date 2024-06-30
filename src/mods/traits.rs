use crate::mods::{*};
use crate::Data;
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
    pub options: Vec<ChoiceOption>
}

pub struct ChoiceOption
{
    pub description: String,
    pub effect: Box<dyn FnOnce(&mut Character)>
}

impl Choice {
    pub fn new(description: String) -> Self
    {
        Choice
        {
            description,
            options: vec![]
        }
    }

    pub fn apply_option(&mut self, character: &mut Character, index: usize)
    {
        (self.options.remove(index).effect)(character);
    }
}