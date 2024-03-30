use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Action
{

}
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Dice
{
    D4, D6, D8, D10, D12, D20, D100
}