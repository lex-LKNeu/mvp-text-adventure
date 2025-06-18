---
title: Making a Silly Little Text Adventure
subtitle: a document of my process
author: Alexander Kimmel
---

I've finally decided to stop being a recluse, commit to a toolset, scrape all the (lowercase-r) rust off of my programming skill and build a portfolio to prove I know what I'm talking about. In my procrastination, I've given quite a few passes to quite a few different languages and decided that (capital-R) Rust is the language I'm going to go with for now:

- has functional programming features I like
- has a strong, static type system, which I also like
- catches footguns early
- the only real wheel-reinventing I have to do is figuring out the lifetime system
- big environment; I have stuff I can do with it

But I need to get back into the swing of things after spending half a decade trying to survive covid-19 and its consequences, so I'm keeping things small and making a minimum-viable-product command-line text adventure.

This is the smallest idea I could come up with that wasn't too trivial:

- three rooms: you start in the middle, one room is to the east, one room is to the west
- there is a locked door between center and west room
- there is a key in the east room
- once you unlock the door and go to the west room, you win

This is done in the typical manner of little commands, like `take key` and `west`. The parser syntax will be simple, `<verb> <object>`. (This is, after all, an MVP.)

Which means I need to write a loop that:

- takes a command input,
- parses it into an actual command (so I don't have to repeat myself for synonyms),
- runs the command against the game state, changing it accordingly,
- and prints the results and a prompt for the next input.

So I need to define:

- a parser
- a world state
- a function interface for changing that world state and generating a response
- a printer for the response

That last one will probably just be a `println!()`, maybe with a naive word wrapper, but I'm putting it separately because I want to be able to plug this same logic into a web interface later.

---

I've decided that the parser will take a string input and output a function that can operate on a `WorldState` object.

This is FP as shit and I'm kinda mad at myself for not sticking to Haskell, but that's water under the bridge, for now at least. Closures should work fine.

My main issue here is that I have commands with more than one active word:

- `inv` is a one-word command
- `go` takes a second word, a direction
- but each direction is a command unto itself: `n` == `go north`

I'll worry about the abbreviations later.

**List of words that need to work:**

- `north`, `south`, `east`, `west`
- `go` each of those directions
- `take <item>`
- `drop <item>`
- `unlock <door>`

I will *not* add abbreviations yet. One task at a time.

---

Backed up! `parse()` takes a string, but does *not* return a function; instead, it returns a `Command` enum. This gives me some extra flexibility in doing what in FP would be monad-like things (specifically command history), and embeds a contract with myself into the type system: if I put in a new command, the compiler will show me all the places where I need to handle the new variant.

If I still have weird one-off behavior for something, I can still throw in a closure and run it.

---

Have a stupid problem: I have more than one kind of item, they share a lot of features, and I forgot to use traits.

So now I gotta double back and re-implement traits for a couple of things. (Specifically, pull `Item` into a trait and make `BasicItem` and `Key` implementations.)

---

Time to implement rooms and doors so the PC can actually move around. (I'm going to hold off on items until this works.)

Current plan:

- make all the rooms with empty door lists
- make all the doors
- have a helper function populate the door lists to match the defined doors

This keeps me from having to define connections between rooms and doors twice.

I had the idea of keeping all the rooms in one big `Vec` and making it a field of the `WorldState`, and so far I'm going with it, but I have one problem: now I can't store a reference to one of the rooms as another field of the `WorldState` struct.

After some poking around (and asking ChatGPT) I found a solution: use indices instead of references. One small problem: if I just have a bunch of `usize`s around, future me is going to forget why I'm using them and try to refactor my way into a wall. So I'm using a type alias: `RoomID`.

---

Current problem: I'm trying to put the world together (four rooms, three doors, one key, one lock).

> This ended up being pretty boring so I abandoned the write-up.

---

Next thing to do: let the PC *move*. I have a `move_pc` function, but all it does is spit back a response that acknowledges the player's direction of movement but does nothing else:

```rust
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
        "You go {}, allegedly. (I haven't actually implemented this yet.)", // TODO
        direction_name
    )
}
```

I'd like the room's description to print when the PC moves into it, at *least* for the first time. That's easy enough to track, though: I just added a `visited` field to the `Room` struct, set the bedroom's to `true` and everyone else gets `false`.

Now I need to write a function that:

- uses the direction given to check the current room's adjacents
- sees if there's a door that way, and if so, if the door's unlocked and open
- if so, move the PC to the other room and update the other room's `visited` field if needed
- return relevant message

... though I might change it so the states are just locked and unlocked; having the door open vs closed doesn't really matter for gameplay.

Update an hour later: this is rapidly turning into a mess of spaghetti, and I'm wondering if the way I set up the Door struct to implement one-way and asymmetrical doors was a mistake. I was probably thinking too far ahead for the problem.

I ended up having to use this horrible, horrible `match` statement:

```rust
let new_room_id = match door.directions.1 {
    None => door.endpoints.1,
    Some(direction) if door.endpoints.0 == current_room.id => door.endpoints.1,
    Some(direction) => door.endpoints.0,
};
```

to figure out which way to go through the doors.

Except I don't even need to do that, because I can just compare the endpoints: if `.0` is the current one, go to `.1`; otherwise, go to `.0`. (Note: this does allow doors that lead back to the start. I may exploit this later.)

Then in testing I found out that I had accidentally input the map upside-down, so I had to edit that. It didn't really matter --- I'm not doing any Crazy Tricks of any sort with the geography, this being four rooms and three doors --- but I wanted to be sure that I expressed my intentions, instead of just retconning a mistake as "totally my intention, trust me" when the fix was pretty trivial.

You know, I'm writing this to be a learning experience, and I sure am doing a lot of learning!