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
