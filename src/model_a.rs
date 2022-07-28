use std::fmt::Display;

trait Roll:Sized+Clone {
  type P: Eq+Clone+Sized;
  type V: Ord+Clone+Sized;
  type T: Ord+Clone+Sized;

  fn player(&self) -> &Self::P;
  fn value(&self) -> &Self::V;
  fn time(&self) -> &Self::T;
}

trait Round: Sized
{
  type P: Eq+Clone+Sized;
  type V: Ord+Clone+Sized;
  type T: Ord+Clone+Sized;
  type R: Roll<P=Self::P, V=Self::V, T=Self::T> + Clone + Sized;

  fn count( &self ) -> usize;
  fn get( &self, index:usize ) -> Option<&Self::R>;
  fn push( &self, player:&Self::P ) -> Result<(Self,Self::R),String>;
}

#[derive(Clone)]
struct BaseRoll<P:Eq+Clone+Sized, V:Ord+Clone+Sized, T:Ord+Clone+Sized> {
  player:P,
  value:V,
  time:T,
}
impl<P:Eq+Clone+Sized, V:Ord+Clone+Sized, T:Ord+Clone+Sized> Roll for BaseRoll<P,V,T> {
  type P=P;
  type V=V;
  type T=T;

  fn player(&self) -> &Self::P { &self.player }
  fn value(&self) -> &Self::V { &self.value }
  fn time(&self) -> &Self::T { &self.time }
}

#[derive(Clone)]
struct BaseRound<P,V,T,NOW,RND> 
where
  P:Eq+Clone+Sized,
  V:Ord+Clone+Sized,
  T:Ord+Clone+Sized,
  NOW: FnMut()->T+Clone,
  RND: FnMut()->V+Clone,
{
  rolls: Vec<BaseRoll<P,V,T>>,
  now:NOW,
  rnd:RND,
}

impl<P,V,T,NOW,RND> Round for BaseRound<P,V,T,NOW,RND> 
where
  P:Eq+Clone+Sized,
  V:Ord+Clone+Sized,
  T:Ord+Clone+Sized,
  NOW: FnMut()->T+Clone,
  RND: FnMut()->V+Clone,
{
  type P=P;
  type V=V;
  type T=T;
  type R=BaseRoll<P,V,T>;

  fn count( &self ) -> usize { self.rolls.len() }
  fn get( &self, index:usize ) -> Option<&Self::R> { self.rolls.get(index) }
  fn push( &self, player:&Self::P ) -> Result<(Self,Self::R),String> {
    let mut round = Clone::clone(self);
    
    let roll = BaseRoll {
      player: player.clone(),
      value: (round.rnd)(),
      time: (round.now)(),
    };

    round.rolls.push(roll.clone());
    Ok( (round, roll) )
  }
}

impl<P,V,T,NOW,RND> std::fmt::Display for BaseRound<P,V,T,NOW,RND> 
where
  P:Eq+Clone+Sized+Display,
  V:Ord+Clone+Sized+Display,
  T:Ord+Clone+Sized+Display,
  NOW: FnMut()->T+Clone,
  RND: FnMut()->V+Clone,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut s = String::new();
    s.push_str( &format!("aa") );

    writeln!(f,"{}",s)
  }
}

#[test]
fn test1() {
  use std::time::{Instant,Duration};
  use rand::Rng;

  let mut round = BaseRound {
    rolls: Vec::<BaseRoll<u8,u8,Instant>>::new(),
    now: || Instant::now(),
    rnd: || 2,
  };

  round.push( &(1u8) );
}

