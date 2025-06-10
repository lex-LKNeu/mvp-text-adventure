use std::io;

mod types;
// I'll prune this once I know what I'm actually using:
use types::{WorldState, Command, Direction, ItemTrait, Item, Key, Room, DoorState, Door};

fn main() -> io::Result<()> {
    let mut world_state = WorldState { quit: false };
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let command: Command = parse(&input);
        let result = run(command, &mut world_state);
        println!("{}", result);
        if world_state.quit {
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
            ws.quit = true;
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
    format!(
        "You go {}, allegedly. (I haven't actually implemented this yet.)",
        direction_name
    )
}
