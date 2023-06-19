use std::collections::HashMap;
use std::io;

#[derive(Clone)]
struct Item {
    name: String,
    description: String,
}

struct Room {
    title: String,
    description: String,
    items: Vec<Item>,
    exits: HashMap<String, usize>
}

struct Player {
    inventory: Vec<Item>,
}

fn get_user_input() -> Result<String, io::Error> {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn display_room(room: &Room) {
    println!("{}\n\n{}", room.title, room.description);
    for item in &room.items {
        println!("{}", item.name);
    }
}

fn load_rooms() -> Vec<Room> {
    vec![
        Room {
            title: "An empty room".to_string(),
            description: "You are in an empty room. How boring.".to_string(),
            items: vec![Item { name: "dagger".to_string(), description: "a small dagger".to_string() }],
            exits: HashMap::from([("east".to_string(), 1)])
        },
        Room {
            title: "A boring room".to_string(),
            description: "You are in a boring room. How empty.".to_string(),
            items: vec![
                Item { name: "apple".to_string(), description: "a rotten apple".to_string() },
                Item { name: "coin".to_string(), description: "a gold coin".to_string() }
            ],
            exits: HashMap::from([("west".to_string(), 0)])
        }
    ]
}

fn main() -> Result<(), io::Error> {
    let mut player = Player { inventory: vec![Item { name: "beets".to_string(), description: "a pile of beets".to_string() }] };
    let mut rooms = load_rooms();
    let mut cur_room = 0;

    display_room(&rooms[cur_room]);
    loop {
        let input = get_user_input()?;
        let mut input = input.split_whitespace();
        let cmd = input.next().unwrap();
        match cmd {
            "quit" => break,
            "look" => display_room(&rooms[cur_room]),
            "take" => {
                let target = input.next().unwrap();
                let room_items = &rooms[cur_room].items;
                let room_target = room_items.into_iter().enumerate().filter(|(_, f)| { f.name == target }).next();
                match room_target {
                    None => println!("You don't see a {} here", target),
                    Some((i, t)) => {
                        println!("You take {}", t.description);
                        player.inventory.push(rooms[cur_room].items.remove(i));
                    }
                }
            }
            "drop" => {
                let target = input.next().unwrap();
                let inventory = &player.inventory;
                let inv_target = inventory.into_iter().enumerate().filter(|(_, f)| { f.name == target }).next();
                match inv_target {
                    None => println!("You are not carrying a {}", target),
                    Some((i, t)) => {
                        println!("You drop {}", t.description);
                        rooms[cur_room].items.push(player.inventory.remove(i));
                    }
                }
            }
            "inventory" => {
                show_player_inventory(&mut player);
            },
            dir @ ("east" | "west") => {
                match rooms[cur_room].exits.get(dir) {
                    None => println!("You cannot go that way"),
                    Some(new_room_index) => {
                        cur_room = new_room_index.clone();
                        display_room(&rooms[cur_room])
                    }
                }
            }
            _ => println!("I don't understand.")
        }
    }
    Ok(())
}

fn show_player_inventory(player: &mut Player) {
    if player.inventory.len() > 0 {
        println!("You are carrying the following items: ");
        for item in &player.inventory {
            println!("{}", item.name);
        }
    } else {
        println!("You are not carrying anything");
    }
}
