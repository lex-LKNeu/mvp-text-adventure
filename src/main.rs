use std::io;

struct WorldState {
    quit: bool,
} // TODO: actually populate this

type Command = fn(&mut WorldState) -> String;

fn main() -> io::Result<()> {
    let mut world_state = WorldState { quit: false };
    loop {
        let mut input = String::new();
        // take a line of input then extract the last word from it
        io::stdin().read_line(&mut input)?;
        // run it through a parser
        let command = parse(&input);
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
        return Box::new(|_: &mut WorldState| String::from("..."));
    }

    let first_word = words[0];

    // check words against a database of commands or something?
    // one big match, for now
    let command: Command = match first_word {
        "xyzzy" => |_| String::from("Very funny."),
        "quit" => |ws| {
            ws.quit = true;
            String::from("Goodbye!")
        },
        _ => |_| String::from("I don't know what that means."),
    };

    Box::new(command)
}
