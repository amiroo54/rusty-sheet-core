use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Action
{
    pub lvl: u8,
    pub description: String,
    pub action_type: ActionType
}

impl Action
{
    pub fn new(description: String, lvl: u8, action_type: ActionType) -> Self
    {
        Action{description, lvl, action_type}
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum ActionType
{
    Action, BonusAction, Reaction, Trait
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Dice
{
    D4, D6, D8, D10, D12, D20, D100
}