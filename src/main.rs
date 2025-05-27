use std::io;

struct WorldState {
    quit: bool,
} // TODO: actually populate this

type Command = Box<dyn Fn(&mut WorldState) -> String>;

fn main() -> io::Result<()> {
    let mut world_state = WorldState { quit: false };
    loop {
        let mut input = String::new();
        // take a line of input then extract the last word from it
        io::stdin().read_line(&mut input)?;
        // run it through a parser
        let command: Command = parse(&input);
        // then operate on the world with the resulting closure
        let result = command(&mut world_state);
        // and print back the results
        println!("{}", result);
        if world_state.quit {
            break;
        }
    }
    Ok(())
}

fn parse(input: &str) -> Box<dyn Fn(&mut WorldState) -> String> {
    // break up input into words
    let words: Vec<&str> = input.split_whitespace().collect();

    if words.is_empty() {
        return Box::new(|_| String::from("..."));
    }

    // check words against a database of commands or something?
    // one big match, for now
    let command: Command = match words[0] {
        "xyzzy" => Box::new(|_: &mut WorldState| String::from("Very funny.")),
        "quit" => Box::new(|ws: &mut WorldState| {
            ws.quit = true;
            String::from("Goodbye!")
        }),
        _ => Box::new(|_: &mut WorldState| String::from("I don't know what that means.")),
    };

    command
}
