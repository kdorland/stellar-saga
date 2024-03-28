pub mod models;

use models::{RoomGraph,Room,Exit};

fn main() {

    let mut room_graph = RoomGraph::new();

    let room1_id = room_graph.create_room("Room 1", "This is room 1");
    let room2_id = room_graph.create_room("Room 2", "This is room 2");
    let room3_id = room_graph.create_room("Room 3", "This is room 3");
    let room4_id = room_graph.create_room("Room 4", "This is room 4");

    room_graph.create_exit_both_ways(room1_id, room2_id);
    room_graph.create_exit_both_ways(room1_id, room3_id);
    room_graph.create_exit_both_ways(room2_id, room3_id);
    room_graph.create_exit_both_ways(room3_id, room4_id);

    println!("Room Graph: {:#?}", room_graph);

    println!("Room 1: {:#?}", room_graph.get_room(room1_id));

    println!("Exits from Room 1: {:#?}", room_graph.get_exits_from_room(room1_id));

    room_graph.draw_graph();
}
