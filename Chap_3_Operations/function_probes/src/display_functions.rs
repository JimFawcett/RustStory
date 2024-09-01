// function_probes

#![allow(unused_imports)]
#![allow(dead_code)]
use std::any::type_name;
use std::any::Any;
use std::mem::size_of;
use std::fmt::Debug;
//use display::*;

/////////////////////////////////////////////////
// Useful functions

/*-----------------------------------------------
   Accepts either String or str
*/
pub fn shows<S: Into<String>>(s:S) {
  print!("{}",s.into());
}
/*---------------------------------------------------------
  make indented line
*/
fn as_line<S: AsRef<str> + std::fmt::Display>(s:S) -> String {
  format!("  {s}\n")
}
/*-------------------------------------------------------------
   Display message and value
*/
pub fn show<T: Debug>(msg: &str, t: &T) {
  print!("{}", as_line(str_show(msg, t)));
}
pub fn str_show<T: Debug>(msg: &str, t: &T) -> String {
  format!("{}{:?}", msg, t)
}
 /*-------------------------------------------------------------
   show value
   - expects T to implement Debug
*/
pub fn show_value<T: Debug>(value: &T) {
  print!("  {}\n", str_show_value(value));
}
pub fn str_show_value<T: Debug>(value: &T) -> String {
  format!("value: {:?}", value)
}
/*-------------------------------------------------------------
 show type name
*/
pub fn show_type<T>(_value: &T) {
  print!("  {}\n", str_show_type(_value));
}
pub fn str_show_type<T>(_value: &T) -> String {
  let name = std::any::type_name::<T>();
  format!("TypeId: {}, size: {}", name, size_of::<T>())
}
/*-------------------------------------------------------------
   show type name and value
   - expects T to implement Debug
   - see #[define(Debug)] attributes, above
*/
pub fn log<T: Debug>(value: &T) {
  let name = type_name::<T>();
  println!("  TypeId: {}, size: {}", name, size_of::<T>());
  println!("  value:  {:?}", value);
}
pub fn str_log<T: Debug>(value: &T) -> String {
  let name = type_name::<T>();
  let mut st = format!("  TypeId: {}, size: {}\n", name, size_of::<T>());
  let st1 = format!("  value:  {:?}\n", value);
  st.push_str(&st1);
  st
}
/*---------------------------------------------------------
  display s with underline where s is String or &str
  - AsRef<str> allows function to accept anything that
    can be converted to &str
*/
pub fn show_title<S: AsRef<str>>(s: S) 
{
  let stmp = "-".to_string().repeat(s.as_ref().len() + 2);
  print!("{stmp}\n");
  print!(" {}\n", s.as_ref());
  print!("{stmp}\n");
}
pub fn show_label<S: AsRef<str>>(s: S) 
{
  print!(" {}\n", s.as_ref());
  let stmp = "-".to_string().repeat(s.as_ref().len() + 2);
  print!("{stmp}\n");
}
pub fn show_op<S: Into<String>>(s: S) {
  let tmp: String = "--- ".to_string() + &s.into() + " ---";
  print!("{tmp}\n");
}
/*-------------------------------------------------------------
   show line with len hyphens
*/
pub fn separator(len: u8) {
  let mut s = String::new();
  for _i in 1..len + 2 {
      s.push('-');
  }
  println!(" {}", s);
}
// /*-------------------------------------------------------------
//    Display underlined main title on console
// */
// pub fn main_title(msg: &str) {
//   print!("\n  {}", msg);
//   let s = "=".repeat(msg.len() + 2);
//   print!("\n {}\n", s);
// }
// /*-------------------------------------------------------------
//  Display underlined sub title on console
// */
// pub fn sub_title(msg: &str) {
//   print!("\n  {}", msg);
//   let s = "-".repeat(msg.len() + 2);
//   print!("\n {}\n", s);
// }
/*-------------------------------------------------------------
 push a single newline to console
*/
pub fn putln() {
  println!();
}
/*-------------------------------------------------------------
 pust n newlines to console
*/
pub fn putlns(n: usize) {
  let s = "\n".repeat(n);
  print!("{}", s);
}
