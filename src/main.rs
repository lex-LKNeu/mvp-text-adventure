#![allow(dead_code)] // TEMP

use std::io;

mod types;

use types::{WorldState, Command, Direction, GameStatus, Room, DoorState, init_world};

fn main() -> io::Result<()> {
    let mut world_state = init_world();
    println!("You are in {}", world_state.rooms[world_state.pc_loc].description);
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let command: Command = parse(&input);
        let result = run(command, &mut world_state);
        println!("{}", result);
        if world_state.status == GameStatus::Quit {
            break;
        }
    }
    Ok(())
}

fn parse(input: &str) -> Command {
    let words: Vec<&str> = input.split_whitespace().collect();

    if words.is_empty() {
        return Command::Empty;
    }

    let command: Command = match words[0] {
        "east" => Command::Go(Direction::East),
        "west" => Command::Go(Direction::West),
        "north" => Command::Go(Direction::North),
        "south" => Command::Go(Direction::South),
        "go" => {
            if words.len() < 2 {
                Command::Go(Direction::Unknown)
            } else {
                match words[1] {
                    "east" => Command::Go(Direction::East),
                    "west" => Command::Go(Direction::West),
                    "north" => Command::Go(Direction::North),
                    "south" => Command::Go(Direction::South),
                    _ => Command::Go(Direction::Unknown),
                }
            }
        }
        "xyzzy" => Command::Xyzzy,
        "quit" => Command::Quit,
        "exit" => Command::Quit,
        _ => Command::Unknown,
    };

    command
}

fn run(command: Command, ws: &mut WorldState) -> String {
    match command {
        Command::Go(direction) => move_pc(direction, ws),
        Command::Xyzzy => "Very funny.".to_string(),
        Command::Empty => "...".to_string(),
        Command::Quit => {
            ws.status = GameStatus::Quit;
            "Goodbye!".to_string()
        }
        Command::Unknown => "Unknown command.".to_string(),
    }
}

fn move_pc(direction: Direction, ws: &mut WorldState) -> String {
    let direction_name = match direction {
        Direction::Unknown => {
            return "Go where?".to_string();
        }
        Direction::North => "north",
        Direction::South => "south",
        Direction::East => "east",
        Direction::West => "west",
    };

    let current_room: &Room = &ws.rooms[ws.pc_loc];
    if let Some(door) = current_room.doors.get(&direction) {
        // I'm getting to it!
        let door = &ws.doors[*door];
        if door.state == DoorState::Locked {
            return format!("The door is locked.");
        }

        let new_room_id = if door.endpoints.0 == current_room.id {
            door.endpoints.1
        } else {
            door.endpoints.0
        };

        ws.pc_loc = new_room_id;

        format!("You go {}.\n\nYou are in {}", direction_name, ws.rooms[ws.pc_loc].description)
    } else {
        format!("You can't go that way.")
    }
}