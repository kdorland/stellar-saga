#[derive(Debug)]
pub struct RoomGraph {
    pub rooms: Vec<Room>,
    pub exits: Vec<Exit>,
}

#[derive(Debug)]
pub struct Room {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub exits: Vec<i64>,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug)]
pub struct Exit {
    pub id: i64,
    pub to_room_id: i64,
}

impl RoomGraph {
    pub fn new() -> RoomGraph {
        RoomGraph {
            rooms: Vec::new(),
            exits: Vec::new(),
        }
    }

    pub fn create_room(&mut self, name: &str, description: &str) -> i64{
        let room = Room {
            id: self.rooms.len() as i64,
            name: name.to_string(),
            description: description.to_string(),
            exits: Vec::new(),
        };

        self.rooms.push(room);

        self.rooms.len() as i64 - 1
    }

    pub fn create_exit(&mut self, from_room_id: i64, to_room_id: i64) {
        let exit = Exit {
            id: self.exits.len() as i64,
            to_room_id: to_room_id,
        };

        self.exits.push(exit);

        let room = &mut self.rooms[from_room_id as usize];
        room.exits.push(self.exits.len() as i64 - 1);
    }

    pub fn create_exit_both_ways(&mut self, room1_id: i64, room2_id: i64) {
        self.create_exit(room1_id, room2_id);
        self.create_exit(room2_id, room1_id);
    }

    pub fn get_room(&self, room_id: i64) -> &Room {
        &self.rooms[room_id as usize]
    }

    pub fn get_exit(&self, exit_id: i64) -> &Exit {
        &self.exits[exit_id as usize]
    }

    pub fn get_exits_from_room(&self, room_id: i64) -> Vec<&Exit> {
        let room = &self.rooms[room_id as usize];
        room.exits.iter().map(|&exit_id| self.get_exit(exit_id)).collect()
    }

    pub fn draw_graph(&self) {
        for room in &self.rooms {
            println!("Room: {}", room.name);
            for exit in &room.exits {
                let exit = self.get_exit(*exit);
                let to_room = self.get_room(exit.to_room_id);
                println!("  -> {}", to_room.name);
            }
        }
    }    

}
