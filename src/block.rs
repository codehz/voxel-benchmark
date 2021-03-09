use std::fmt::Display;

use enum_map::Enum;

use crate::SolidBlockDefinition;

pub trait BlockId:
    Enum<SolidBlockDefinition> + Display + PartialEq + Eq + Clone + Copy + 'static
{
}

impl<T> BlockId for T where
    T: Enum<SolidBlockDefinition> + Display + PartialEq + Eq + Clone + Copy + 'static
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Block<Id: BlockId> {
    Empty,
    Solid { id: Id },
}

impl<Id: BlockId> Display for Block<Id> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Block::Empty => write!(f, "<empty>"),
            Block::Solid { id } => write!(f, "{}", id),
        }
    }
}

impl<Id: BlockId> Default for Block<Id> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<Id: BlockId> Block<Id> {
    pub fn is_empty(&self) -> bool {
        match self {
            &Block::Empty => true,
            _ => false,
        }
    }
}
