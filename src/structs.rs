use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Deserialize)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}