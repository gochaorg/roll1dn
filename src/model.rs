use std::sync::{Mutex, Arc};
use std::collections::{HashMap, HashSet};
use std::time::{Instant,Duration};
use rand::Rng;

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
  /// Создание комнаты
  pub fn create( &mut self, name:&str ) -> Result<(Room,u32),String> { todo!() }

  /// Удаление комнаты
  pub fn delete( &mut self, name:&str ) -> Result<u32,String> { todo!() }

  /// Получение списка комнат
  pub fn list( &self ) -> Vec<Room> { todo!() }

  /// Обновление комнаты
  pub fn update( &mut self, name:&str, room:Room ) -> Result<u32,String> { todo!() }

  /// Получение комнаты по ее имени
  pub fn get_mut( &mut self, name:&str ) -> Option<&mut Arc<Box<Room>>> { 
    self.rooms_by_name.get_mut(name)
  }
}

#[derive(Clone)]
pub struct Room {
  /// Название комнаты
  pub name: String,

  /// минимальное кол-во игроков в раунде
  pub players_min: u8,

  /// игроки в данной комнате
  pub players: Vec<String>,

  /// Минимальное значение кубика
  pub value_from: u8,

  /// Максимальное значение кубика
  pub value_to: u8,

  /// максимальное кол-во победителей
  pub max_winners: u8
}

/// Раунд игры
#[derive(Debug)]
pub struct Round<R>
{
  /// Броски кубика
  pub rolls: Vec<Roll>,

  /// Минимальное значение кубика
  pub value_from: u8,

  /// Максимальное значение кубика
  pub value_to: u8,

  /// Время начала раунда
  pub started: Instant,

  /// Время завершения раунда
  pub finished: Option<Instant>,

  /// Ограничение раунда по продолжительности
  pub time_limit: Duration,

  /// Минимальное кол-во участников для корректного завершения раунда
  pub min_players: u8,

  /// Максимальное кол-во участников в раунде
  pub max_players: u8,

  /// Игроки которые допускаются в раунде
  pub allowed_players: HashSet<String>,

  /// Победившие игроки
  pub winners: HashSet<String>,

  /// максимальное кол-во победителей
  pub max_winners: u8,

  /// конфликтующие игроки, которые должны сделать дополнительный ход
  pub conflict: HashSet<String>,

  /// генератор случайных чисел
  pub rnd: Box<R>,
}

/// Бросок кубика игрока
#[derive(Debug)]
pub struct Roll {
  /// Время броска
  pub time: Instant,

  /// Игрок
  pub player: String,

  /// Выпавшее значение
  pub value: u8
}

impl Round<rand::rngs::ThreadRng> {
  pub fn new() -> Self {
    Self { 
      rolls: Vec::new(), 
      value_from: 1, 
      value_to: 12, 
      started: Instant::now(), 
      finished: None, 
      time_limit: Duration::from_secs(60*60), 
      min_players: 0, 
      max_players: 0, 
      allowed_players: HashSet::new(), 
      winners: HashSet::new(), 
      max_winners: 0, 
      conflict: HashSet::new(), 
      rnd: Box::new(rand::thread_rng())
    }
  }

  pub fn allow( &mut self, player: &str ) -> Result<(),String> { todo!() }
  pub fn deny( &mut self, player: &str ) -> Result<(),String> { todo!() }
  pub fn 
}

pub fn new_round() -> Round<rand::rngs::ThreadRng> { Round::new() }

#[test]
fn test_roll() {
  let mut round = new_round();
  println!("{:?}", round)
}