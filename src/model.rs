use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::time::{Instant,Duration};

pub struct AppState {
  pub users: Mutex<Players>,
  pub rooms: Mutex<Rooms>
}

impl AppState {
    pub fn new () -> AppState {
      AppState {
        users: Mutex::new(
            Players {
                id_seq: 0,
                users: HashMap::new()    
            }            
        ),
        rooms: Mutex::new(
          Rooms {
            id_seq: 0,
            rooms: HashMap::new(),
            rooms_by_name: HashMap::new(),
            rooms_by_id: HashMap::new(),
          }
        )
      }
    }
}

pub struct Players {
  pub id_seq: u32,
  pub users: HashMap<String,u32>
}

impl Players {
  pub fn create( &mut self, name:String ) -> Result<u32,String> {
    if self.users.contains_key(&name) {
      Err(String::from("player already defined"))
    }else{
      self.id_seq += 1;
      let id = self.id_seq;
      self.users.insert(name.clone(), id);
      Ok(id)
    }
  }
}

pub struct Rooms {
  pub id_seq: u32,
  pub rooms: HashMap<Arc<Box<Room>>,u32>,
  pub rooms_by_name: HashMap<String, Arc<Box<Room>>>,
  pub rooms_by_id: HashMap<u32,Arc<Box<Room>>>
}

impl Rooms {
  pub fn create( &mut self, name:&str ) -> Result<(Room,u32),String> { todo!() }
  pub fn delete( &mut self, name:&str ) -> Result<u32,String> { todo!() }
  pub fn list( &self ) -> Vec<Room> { todo!() }
  pub fn update( &mut self, name:&str, room:Room ) -> Result<u32,String> { todo!() }
  pub fn find_mut( &mut self, name:&str ) -> Option<&mut Arc<Box<Room>>> { 
    self.rooms_by_name.get_mut(name)
  }
}

#[derive(Clone)]
pub struct Room {
  pub name: String,
  pub players_min: u8,
  pub players: Vec<String>
}

pub struct Round {
  pub rolls: Vec<Roll>,
  pub value_from: u8,
  pub value_to: u8,
  pub started: Instant,
  pub finished: Option<Instant>,
  pub time_limit: Duration,
  pub min_players: u8,
  pub max_players: u8,
  pub allowed_players: Vec<String>,
  pub winners: Vec<String>,
  pub conflict: Vec<String>,
}

pub struct Roll {
  pub time: Instant,
  pub player: String,
  pub value: u8
}