#![allow(dead_code)] // TEMP

use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum Command {
    Go(Direction),
    Quit,
    Xyzzy,
    Unknown,
    Empty,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Unknown,
}

#[derive(PartialEq, Eq)]
pub enum GameStatus {
    Ongoing,
    Won,
    Lost,
    Quit,
}

#[derive(PartialEq, Eq)]
pub enum DoorState {
    Open,
    Closed,
    Locked,
}

pub type RoomID = usize;
pub type DoorID = usize;
pub type ItemID = usize;

pub struct WorldState {
    pub status: GameStatus,
    pub pc_loc: RoomID,
    pub rooms: Vec<Room>,
    pub doors: Vec<Door>,
    pub items: Vec<Box<dyn Item>>,
}

// TODO: actually populate this

pub struct BasicItem {
    pub name: String,
    pub description: String,
}

pub struct Key {
    pub item: BasicItem,
    pub opens: Vec<DoorID>,
}

pub struct Room {
    pub id: RoomID,
    pub name: String,
    pub description: String,
    pub items: Vec<Box<dyn Item>>,
    pub doors: HashMap<Direction, DoorID>,
    pub visited: bool,
}

pub struct Door {
    pub id: DoorID,
    pub description: String,
    pub endpoints: (RoomID, RoomID),
    // if directions.1 is None, it's a one-way door
    pub directions: (Direction, Option<Direction>),
    pub state: DoorState,
    pub key: Option<Box<dyn Item>>, // if None, door cannot be locked --- not sure how to enforce that
}

pub trait Item {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

impl Item for BasicItem {
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
}

impl Item for Key {
    fn name(&self) -> &str {
        &self.item.name
    }
    fn description(&self) -> &str {
        &self.item.description
    }
}

// returns the starting WorldState
pub fn init_world() -> WorldState {
    let rooms = vec![
        Room {
            id: 0,
            name: "living room".to_string(),
            description: "a small living room area, with a dining room table off to one corner.".to_string(),
            items: vec![],
            doors: HashMap::<Direction, DoorID>::new(),
            visited: false,
        },
        Room {
            id: 1,
            name: "kitchen".to_string(),
            description: "a galley-style kitchen.".to_string(),
            items: vec![],
            doors: HashMap::<Direction, DoorID>::new(),
            visited: false,
        },
        Room {
            id: 2,
            name: "bedroom".to_string(),
            description: "a bedroom. The bed tempts.".to_string(),
            items: vec![],
            doors: HashMap::<Direction, DoorID>::new(),
            visited: true,
        },
        Room {
            id: 3,
            name: "outside".to_string(),
            description: "the sweet, fresh air of victory! (Or is that pollen.)".to_string(),
            items: vec![],
            doors: HashMap::<Direction, DoorID>::new(),
            visited: false,
        }
    ];
    let doors = vec![
        Door {
            id: 0,
            description: "the front door.".to_string(),
            endpoints: (0, 3),
            directions: (Direction::South, Some(Direction::North)),
            state: DoorState::Closed,
            key: None,
        },
        Door {
            id: 1,
            description: "an interior door.".to_string(),
            endpoints: (0, 2),
            directions: (Direction::West, Some(Direction::East)),
            state: DoorState::Closed,
            key: None,
        },
        Door {
            id: 2,
            description: "an interior door.".to_string(),
            endpoints: (0, 1),
            directions: (Direction::North, Some(Direction::South)),
            state: DoorState::Closed,
            key: None,
        },
    ];
    let mut ws = WorldState {
        status: GameStatus::Ongoing,
        pc_loc: 2,
        rooms,
        doors,
        items: vec![
            Box::new(Key {
                item: BasicItem {
                    name: "key".to_string(),
                    description: "the key! the key!".to_string(),
                },
                opens: vec![0],
            }),
        ],
    };

    // link doors to rooms
    for door in &ws.doors {
        if let Some(room) = ws.rooms.iter_mut().find(|r| r.id == door.endpoints.0) {
            room.doors.insert(door.directions.0, door.id);
        }
        if let Some(back) = door.directions.1 {
            if let Some(room) = ws.rooms.iter_mut().find(|r| r.id == door.endpoints.1) {
                room.doors.insert(back, door.id);
            }
        }
    }
    ws
}
