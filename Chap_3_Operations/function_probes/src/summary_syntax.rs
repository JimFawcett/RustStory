
use std::fmt::Debug;
use std::default::Default;
use std::sync::atomic::{ AtomicI32, Ordering };

pub fn f<T: Debug>(t:T) {
  println!("pass by value: {t:?}");
}

pub fn g<T: Debug>(rt:&T) {
  println!("pass by ref: {rt:?}");
}

pub fn h<T: Default + Debug>(_t:&T) -> T {
  let u = T::default();
  println!("returns default value: {u:?}");
  u
}

#[derive(Debug)]
pub struct Counter {
  value: AtomicI32,
}
impl Counter {
  pub fn new() -> Counter {
    Counter {
      value:0.into(),
    }
  }
  pub fn incr(&mut self) {
    self.value.fetch_add(1, Ordering::SeqCst);
  }
  pub fn decr(&mut self) {
    self.value.fetch_sub(1, Ordering::SeqCst);
  }
  pub fn count(&self) -> i32 {
    self.value.load(Ordering::SeqCst)
  }
}

enum Direction { Up, Down, Stay }

pub fn demo_syntax() {
  f(3.1415927);
  g(&"a literal string");
  let v = vec![1, 2, 3];
  let mut r = h(&v);
  r.push(42);
  println!("{r:?}");
  let mut cntr = Counter::new();
  let mut l = |dir:Direction| {
    match dir {
      Direction::Up => { cntr.incr(); r.push(cntr.count()); },
      Direction::Down => { cntr.decr(); r.push(cntr.count()); },
      Direction::Stay => { r.push(cntr.count()); },
    }
    // if let Direction::Up = dir {
    //   cntr.incr();
    // }
    // else {
    //   cntr.decr();
    // }
    // r.push(cntr.count()); 
    println!("{:?}", r);
  };
  l(Direction::Up); 
  l(Direction::Up);
  l(Direction::Down);
  l(Direction::Stay);
  l(Direction::Up);
}