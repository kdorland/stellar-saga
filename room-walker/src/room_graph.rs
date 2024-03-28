#[derive(Debug)]
pub struct RoomGraph {
    pub rooms: Vec<Room>,
}

#[derive(Debug)]
pub struct Room {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub exit_room_ids: Vec<i64>,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl RoomGraph {
    pub fn new() -> RoomGraph {
        RoomGraph {
            rooms: Vec::new()
        }
    }

    pub fn create_room(&mut self, name: &str, description: &str) -> Result<i64, String> {
        let room = Room {
            id: self.rooms.len() as i64,
            name: name.to_string(),
            description: description.to_string(),
            exit_room_ids: Vec::new(),
        };

        let id = room.id;

        self.rooms.push(room);

        Ok(id)
    }

    pub fn create_exit(&mut self, from_room_id: usize, to_room_id: i64) -> Result<(), String> {
        match self.rooms.get_mut(from_room_id) {
            Some(from_room) => {
                from_room.exit_room_ids.push(to_room_id);
                Ok(())
            },
            None => Err(format!("Room with ID {} not found", from_room_id)),
        }
    }

    pub fn create_exit_both_ways(&mut self, room1_id: i64, room2_id: i64) -> Result<(), String> {
        self.create_exit(room1_id as usize, room2_id)?;
        self.create_exit(room2_id as usize, room1_id)
    }

    pub fn get_room(&self, room_id: i64) -> &Room {
        &self.rooms[room_id as usize]
    }

}
