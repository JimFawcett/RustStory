// aggr+probes
#![allow(unused_imports)]
#![allow(dead_code)]
use std::fmt::{ Debug, Display };
use std::mem::size_of;

/*-------------------------------------------------------------
  Display Functions
  - 
-------------------------------------------------------------*/
/*-------------------------------------------------------------
   show value
   - expects T to implement Debug
*/
pub fn show_value<T: Debug>(value: &T) {
  println!("  value: {:?}", value);
}
pub fn str_show_value<T: Debug>(value: &T) -> String {
  format!("  value: {:?}", value)
}
/*-------------------------------------------------------------
 show type name
*/
pub fn show_type<T>(_value: &T) {
  let name = std::any::type_name::<T>();
  println!("  TypeId: {}, size: {}", name, size_of::<T>());
}
pub fn str_show_type<T>(_value: &T) -> String {
  let name = std::any::type_name::<T>();
  format!("  TypeId: {}, size: {}", name, size_of::<T>())
}
/*---------------------------------------------------------
  display s with underline where s is String or &str
  - AsRef<str> allows function to accept anything that
    can be converted to &str
*/
pub fn show_title<S: AsRef<str>>(s: S) 
{
  let stmp = "-".to_string().repeat(s.as_ref().len() + 2);
  println!("\n{stmp}");
  println!(" {}", s.as_ref());
  println!("{stmp}\n");
}
pub fn show_label<S: AsRef<str>>(s: S) 
{
  println!(" {}", s.as_ref());
  let stmp = "-".to_string().repeat(s.as_ref().len() + 2);
  println!("{stmp}");
}
pub fn show_op<S: Into<String>>(s: S) {
  let tmp: String = "--- ".to_string() + &s.into() + " ---";
  println!("{tmp}");
}
/*---------------------------------------------------------
  Demonstrate stdlib data types
*/

fn main() {

  show_title("Demonstrate std Library Types");
  use std::collections::{VecDeque, HashMap};

  show_label("std::Vec<T>");

  show_op("let mut vi = vec![1, 2, 3, 2, 1]");
  let mut vi = vec![1, 2, 3, 2, 1];
  show_type(&vi);
  show_value(&vi);
  show_op("vi[1] = -2");
  vi[1] = -2;
  show_value(&vi);
  show_op("vi.push(0)");
  vi.push(0);
  show_value(&vi);
  show_op("vi.insert(1, 42)");
  vi.insert(1, 42);
  show_value(&vi);
  println!();

  show_label("VecDeque<T>");
  show_op("let mut vdeq = VecDeque::<f64>::new()");
  let mut vdeq = VecDeque::<f64>::new();
  show_type(&vdeq);
  show_value(&vdeq);
  show_op("vdeq.push_back(2.5)");
  vdeq.push_back(2.5);
  show_op("vdeq.push_front(1.0)");
  vdeq.push_front(1.0);
  show_value(&vdeq);
  println!();

  show_label("HashMap<K, V>");
  show_op("let mut hm = HashMap::<i32, &str>::new()");
  let mut hm = HashMap::<i32, &str>::new();
  show_type(&hm);
  show_value(&hm);
  show_op("hm.insert(1,\"one\")");
  hm.insert(1,"one");
  show_value(&hm);
  hm.insert(0,"zero");
  show_value(&hm);
  hm.insert(2,"two");
  show_value(&hm);
  hm.insert(-2,"minus two");
  show_value(&hm);
  show_op("hm.remove(&0)");
  hm.remove(&0);
  show_value(&hm);
  /*
    using entry API for HashMap
    - if the key exists then modify the value
      with a closure 
  */
  show_op("hm.entry(1).and_modify(|v| *v = \"the number 1\")");
  hm.entry(1).and_modify(|v| *v = "the number 1");
  show_value(&hm);

  println!("\n  That's all Folks!");
}
