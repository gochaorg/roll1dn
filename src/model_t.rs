use std::fmt::Display;
use std::marker::PhantomData;
use std::time::{Instant,Duration};
use std::collections::{HashMap, HashSet, BTreeSet, BTreeMap};
use rand::Rng;

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct Player(String);

impl Display for Player {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f,"{}", self.0)
  }
}

pub trait Rand<V: Ord> {
  fn rand( &mut self ) -> V;
}

pub trait Now<T> {
  fn now( &self ) -> T;
}

pub trait Roll<V:Ord,T:Ord> : Clone {
  fn player(&self) -> Player;
  fn value(&self) -> V;
  fn time(&self) -> T;
}

#[derive(Clone,Debug)]
pub struct SimpleRoll {
  player: Player,
  value: u8,
  time: Instant
}

impl Roll<u8,Instant> for SimpleRoll {
  fn player(&self) -> Player { self.player.clone() }
  fn value(&self) -> u8 { self.value }
  fn time(&self) -> Instant { self.time }
}

pub trait RoundPlayers {
    fn exists( &self, player: &Player ) -> bool;
    fn insert( &mut self, player: &Player ) -> Result<(),String>;
    fn list_players( &self ) -> Vec<Player>;
}

impl RoundPlayers for HashSet<Player> {
  fn exists( &self, player: &Player ) -> bool { self.contains(player) }
  fn insert( &mut self, player: &Player ) -> Result<(),String> { self.insert(player.clone()); Ok(()) }
  fn list_players( &self ) -> Vec<Player> {
    let mut res = Vec::<Player>::new();
    for p in self.iter() {
      res.push(p.clone())
    }
    res
  }
}

pub trait Rolls {
  type T: Ord;
  type V: Ord;
  type R: Roll<Self::V,Self::T> + Clone;

  fn rolls(&self) -> Vec<Self::R>;
  fn rolls_count(&self) -> usize;
  fn get_roll(&self, idx:usize) -> Option<Self::R>;
  fn push(&mut self, roll:Self::R) -> Result<usize,String>;
}

impl Rolls for Vec<SimpleRoll> {
  type T = Instant;
  type V = u8;
  type R = SimpleRoll;

  fn rolls(&self) -> Vec<Self::R> { self.clone() }
  fn rolls_count(&self) -> usize { self.len() }

  fn get_roll(&self, idx:usize) -> Option<Self::R> {
    //https://stackoverflow.com/questions/44445730/how-to-call-a-method-when-a-trait-and-struct-use-the-same-method-name
    self.as_slice().get(idx).map(|x|x.clone())
  }

  fn push(&mut self, roll:Self::R) -> Result<usize,String> {
    Vec::<SimpleRoll>::push(self, roll);
    Ok(self.len())
  }
}

pub trait Round : Rand<Self::V> + Now<Self::T> + RoundPlayers
{
  type T: Ord;
  type V: Ord;
  type R: Roll<Self::V,Self::T>+Clone;
  type ROLLS: Rolls<T=Self::T, V=Self::V, R=Self::R>;

  fn roll( &self, player:&Player, time: Self::T, value: Self::V) -> Self::R;
  fn rolls( &self ) -> &Self::ROLLS;
  fn rolls_mut( &mut self ) -> &mut Self::ROLLS;
  fn roll_cube( &mut self, player:&Player ) -> Result<Self::R,String> {
    self.insert(player)?;

    let value = self.rand();
    let time = self.now();    
    let roll = self.roll(player, time, value);
    self.rolls_mut().push(roll.clone())?;

    Ok(roll)
  }
}

pub struct SimpleRound {
  rolls: Vec<SimpleRoll>,
  players: HashSet<Player>,
  rnd: rand::rngs::ThreadRng
}

impl Round for SimpleRound {
  type T = Instant;
  type V = u8;
  type R = SimpleRoll;
  type ROLLS = Vec<SimpleRoll>;

  fn rolls( &self ) -> &Self::ROLLS { &self.rolls }
  fn rolls_mut( &mut self ) -> &mut Self::ROLLS { &mut self.rolls }

  fn roll( &self, player:&Player, time: Self::T, value: Self::V) -> Self::R {
    SimpleRoll {
      player: player.clone(),
      value: value,
      time: time
    }
  }
}

impl Rand<u8> for SimpleRound {  
  fn rand( &mut self ) -> u8 {
    self.rnd.gen()
  }
}

impl Now<Instant> for SimpleRound {
  fn now( &self ) -> Instant {
    Instant::now()
  }
}

impl RoundPlayers for SimpleRound {
  fn exists( &self, player: &Player ) -> bool {
    self.players.exists(player)
  }

  fn insert( &mut self, player: &Player ) -> Result<(),String> {
    let s: &mut dyn RoundPlayers = &mut self.players;
    s.insert(player)
  }

  fn list_players( &self ) -> Vec<Player> {
    self.players.list_players()
  }
}

#[test]
fn roll_cude_test() {
  let mut round = SimpleRound {
    rolls: Vec::new(),
    players: HashSet::new(),
    rnd: rand::thread_rng()
  };

  let plyr = Player(String::from("aa"));
  match round.roll_cube(&plyr) {
    Ok(rl0) => {
      println!("roll {:?}", rl0);
      for x in round.list_players().iter() {
        println!("player {x}")
      }
    },
    Err(err) => println!("error {:?}",err)
  }
}

pub struct SRound<V,T,Rs,R,RND,NOW,CR> 
where 
  Rs:Rolls<V=V, T=T, R=R>,
  V:Ord,
  T:Ord,
  R:Roll<V,T>,
  RND: Rand<V>,
  NOW: Now<T>,
  CR: Fn(&Player,T,V)->R,
{
  create_roll: CR,
  rolls: Rs,
  rnd: RND,
  now: NOW,  
  _p : PhantomData<(T,R)>
}

impl<V,T,Rs,R,RND,NOW,CR> Round for SRound<V,T,Rs,R,RND,NOW,CR>
where 
  Rs:Rolls<V=V, T=T, R=R>,
  V:Ord,
  T:Ord,
  R:Roll<V,T>,
  RND: Rand<V>,
  NOW: Now<T>,
  CR: Fn(&Player,T,V)->R,
{
  type T = T;
  type V = V;
  type R = R;
  type ROLLS = Rs;

  fn roll( &self, player:&Player, time: Self::T, value: Self::V) -> Self::R {
    (self.create_roll)(player,time,value)
  }

  fn rolls( &self ) -> &Self::ROLLS {
    &self.rolls
  }

  fn rolls_mut( &mut self ) -> &mut Self::ROLLS { &mut self.rolls }
}

impl<V,T,Rs,R,RND,NOW,CR> RoundPlayers for SRound<V,T,Rs,R,RND,NOW,CR> 
where 
  Rs:Rolls<V=V, T=T, R=R>,
  V:Ord,
  T:Ord,
  R:Roll<V,T>,
  RND: Rand<V>,
  NOW: Now<T>,
  CR: Fn(&Player,T,V)->R,
{
    fn exists( &self, player: &Player ) -> bool {
        todo!()
    }

    fn insert( &mut self, player: &Player ) -> Result<(),String> {
        todo!()
    }

    fn list_players( &self ) -> Vec<Player> {
        todo!()
    }
}

impl<V,T,Rs,R,RND,NOW,CR> Now<T> for SRound<V,T,Rs,R,RND,NOW,CR> 
where 
  Rs:Rolls<V=V, T=T, R=R>,
  V:Ord,
  T:Ord,
  R:Roll<V,T>,
  RND: Rand<V>,
  NOW: Now<T>,
  CR: Fn(&Player,T,V)->R,
{
  fn now( &self ) -> T { self.now.now() }
}

impl<V,T,Rs,R,RND,NOW,CR> Rand<V> for SRound<V,T,Rs,R,RND,NOW,CR> 
where 
  Rs:Rolls<V=V, T=T, R=R>,
  V:Ord,
  T:Ord,
  R:Roll<V,T>,
  RND: Rand<V>,
  NOW: Now<T>,
  CR: Fn(&Player,T,V)->R,
{
  fn rand( &mut self ) -> V { self.rnd.rand() }
}
