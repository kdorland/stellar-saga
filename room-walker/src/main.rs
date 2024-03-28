#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
// TODO: Remove above when done experimenting

extern crate colored; // not needed in Rust 2018+

mod room_graph;
mod player_state;

use std::io;
use colored::*; // Importer alt fra `colored`
use serde_json::Value;
use room_graph::RoomGraph;

fn main() {
    println!("Walk the rooms!");

    let room_graph = parse_rooms_from_json().unwrap();
    let mut player_state = player_state::PlayerState {
        current_room: 0,
    };

    loop {
        let mut input = String::new();

        println!("{}", "\nWhat do you do?".red());

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading text input.");

        // Trim input og konverter til lowercase for nemmere håndtering
        let input = input.trim().to_lowercase();

        // Afslut loop, hvis spilleren skriver "exit"
        if input == "exit" {
            println!("{}", "Goodbye".red());
            break;
        }

        // Parser og reager på input
        let mut parts = input.splitn(2, ' ');
        let command = parts.next().unwrap_or("");
        let args = parts.next().unwrap_or("");

        println!("{}", get_echo_text(&input));
        
        match command {
            "look" => println!("{}", get_room_text(&room_graph, &player_state)),
            "go" => command_go(args, &room_graph, &mut player_state),
            "help" => println!("Commands: look, go, exit"),
            _ => println!("Unknown command: {}", input),
        }
    }    
}

fn parse_rooms_from_json() -> Result<RoomGraph, String> {
    let mut room_graph = RoomGraph::new();

    let json_str = include_str!("../res/rooms.json");
    let rooms: Vec<Value> = serde_json::from_str(json_str).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    for room in rooms {
        let name = room["name"].as_str().ok_or("Room name is not a string")?;
        let description = room["description"].as_str().ok_or("Room description is not a string")?;
        let room_id = room_graph.create_room(name, description)?;
        let exits = room["exit_room_ids"].as_array().ok_or("Exit room IDs is not an array")?;

        for exit in exits {
            let exit_id = exit.as_i64().ok_or("Exit room ID is not an integer")?;
            room_graph.create_exit(room_id as usize, exit_id)?;
        }
    }

    Ok(room_graph)
}

fn get_room_text(room_graph: &RoomGraph, player_state: &player_state::PlayerState) -> String {
    let current_room = room_graph.get_room(player_state.current_room as i64);
    
    let exits = current_room.exit_room_ids.iter().enumerate().map(|(i, exit_id)| {
        let exit_room = room_graph.get_room(*exit_id);
        format!("{}: {}", i, exit_room.name)
    }).collect::<Vec<String>>().join("\n");

    format!("{}\n{}", current_room.name, current_room.description) + &format!("\nExits:\n{}", exits)
}

fn get_echo_text(input: &str) -> String {
    format!("You said: {}", input.green())
}

fn command_go(args: &str, room_graph: &RoomGraph, player_state: &mut player_state::PlayerState) {
    if args.is_empty() {
        println!("Go where?");
    } else {
        // Match the exit id to the room id, and then move the player
        let exit_id = match args.parse::<usize>() {
            Ok(id) => id,
            Err(_) => {
                println!("No such room: {}", args);
                return;
            }
        };

        // Check it exit_id is in the room exit list:
        if exit_id >= room_graph.get_room(player_state.current_room as i64).exit_room_ids.len() {
            println!("No such exit: {}", exit_id);
            return;
        }

        let exit_room_id = room_graph.get_room(player_state.current_room as i64).exit_room_ids[exit_id];
        move_player(player_state, exit_room_id as usize, room_graph);
    }
}

fn move_player(player_state: &mut player_state::PlayerState, room_id: usize, room_graph: &RoomGraph) {
    player_state.current_room = room_id;
    println!("{}", "You move to a new room.".green());
    println!("{}", get_room_text(room_graph, player_state));
}