#[derive(PartialEq, Eq, Hash)]
pub enum Command {
    Go(Direction),
    Quit,
    Xyzzy,
    Unknown,
    Empty,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Unknown,
}

pub struct WorldState {
    pub quit: bool,
} // TODO: actually populate this

pub trait Item {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

pub struct BasicItem {
    pub name: String,
    pub description: String,
}

impl Item for BasicItem {
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
}

pub struct Key {
    pub item: BasicItem,
    pub opens: Vec<Door>,
}

impl Item for Key {
    fn name(&self) -> &str {
        &self.item.name
    }
    fn description(&self) -> &str {
        &self.item.description
    }
}

pub struct Room {
    pub name: String,
    pub description: String,
    pub items: Vec<Item>,
    pub connections: Vec<Door>,
}

pub enum DoorState {
    Open,
    Closed,
    Locked,
}

pub struct Door {
    pub endpoints: (Room, Room),
    // if directions.1 is None, it's a one-way door
    pub directions: (Direction, Option<Direction>),
    pub state: DoorState,
    pub key: Option<Item>, // if None, door cannot be locked
}
