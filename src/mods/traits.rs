use crate::mods::{*};

pub trait Actionable
{
    fn get_actions(&self) -> Vec<&Action>;
}  

pub trait Stateable
{
    fn get_added_stats(&self) -> Vec<&Stats>;
}

pub trait FileLoadable
{
    fn get_json(&self) -> String;
    fn load_from_json(json: &String) -> Self;
}