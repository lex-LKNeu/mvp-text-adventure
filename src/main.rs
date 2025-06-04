use std::io;

struct WorldState {
    quit: bool,
} // TODO: actually populate this

#[derive(PartialEq, Eq, Hash)]
enum Command {
    Go(Direction),
    Quit,
    Xyzzy,
    Unknown,
    Empty,
}

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
    Unknown,
}

fn main() -> io::Result<()> {
    let mut world_state = WorldState { quit: false };
    loop {
        let mut input = String::new();
        // take a line of input then extract the last word from it
        io::stdin().read_line(&mut input)?;
        // run it through a parser
        let command: Command = parse(&input);
        // then operate on the world with the resulting closure
        let result = run(command, &mut world_state);
        // and print back the results
        println!("{}", result);
        if world_state.quit {
            break;
        }
    }
    Ok(())
}

fn parse(input: &str) -> Command {
    // break up input into words
    let words: Vec<&str> = input.split_whitespace().collect();

    if words.is_empty() {
        return Command::Empty;
    }

    // check words against a database of commands or something?
    // one big match, for now
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

    // TODO: how is this parser going to handle two-word commands?

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
