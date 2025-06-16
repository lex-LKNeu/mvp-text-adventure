#![allow(dead_code)] // TEMP

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

pub enum GameStatus {
    Ongoing,
    Won,
    Lost,
    Quit,
}

pub type RoomID = usize;
pub type DoorID = usize;
pub type ItemID = usize;

pub struct WorldState {
    pub quit: bool,
    pub pc_loc: RoomID,
    pub rooms: Vec<Room>,
    pub doors: Vec<Door>,
    pub items: Vec<Box<dyn Item>>,
} // TODO: actually populate this

// returns the starting WorldState
pub fn init_world() -> WorldState {
    WorldState {
        quit: false,
        pc_loc: 0,
        rooms: vec![
            Room {
                name: "living room".to_string(),
                description: "a small living room area, with a dining room table off to one corner.".to_string(),
                items: vec![],
                doors: vec![],
            },
            Room {
                name: "kitchen".to_string(),
                description: "a galley-style kitchen.".to_string(),
                items: vec![],
                doors: vec![],
            },
            Room {
                name: "bedroom".to_string(),
                description: "a bedroom. The bed tempts.".to_string(),
                items: vec![],
                doors: vec![],
            },
            Room {
                name: "outside".to_string(),
                description: "the sweet, fresh air of victory! (Or is that pollen.)".to_string(),
                items: vec![],
                doors: vec![],
            }
        ],
        doors: vec![
            Door {
                description: "the front door.".to_string(),
                endpoints: (0, 3),
                directions: (Direction::North, Some(Direction::South)),
                state: DoorState::Closed,
                key: None,
            },
            Door {
                description: "an interior door.".to_string(),
                endpoints: (0, 2),
                directions: (Direction::East, Some(Direction::West)),
                state: DoorState::Closed,
                key: None,
            },
            Door {
                description: "an interior door.".to_string(),
                endpoints: (0, 1),
                directions: (Direction::South, Some(Direction::North)),
                state: DoorState::Closed,
                key: None,
            },
        ],
        items: vec![
            Box::new(Key {
                item: BasicItem {
                    name: "key".to_string(),
                    description: "the key! the key!".to_string(),
                },
                opens: vec![0],
            }),
        ],
    }
}


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
    pub opens: Vec<DoorID>,
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
    pub items: Vec<Box<dyn Item>>,
    pub doors: Vec<DoorID>,
}

pub enum DoorState {
    Open,
    Closed,
    Locked,
}

pub struct Door {
    pub description: String,
    pub endpoints: (RoomID, RoomID),
    // if directions.1 is None, it's a one-way door
    pub directions: (Direction, Option<Direction>),
    pub state: DoorState,
    pub key: Option<Box<dyn Item>>, // if None, door cannot be locked
}
