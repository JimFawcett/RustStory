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

/*-- create and display basic aggregates -*/

fn basic_aggr() {
  show_title("Demonstrate Rust Aggregates");

  /*-- array --*/
  show_label("arrays");
  show_op("let mut arr:[i32; 5] = [1, 2, 3, 4, 5]");
  let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
  show_type(&arr);
  show_value(&arr);
  show_op("arr[1] = -2");
  arr[1] = -2;
  show_value(&arr);
  println!();

  /*-- slice --*/
  show_label("slices");
  show_op("let slc = &mut arr[1..4]");
  let slc = &mut arr[1..4];
  show_type(&slc);
  show_value(&slc);
  show_op("slc[0] = 0");
  slc[0] = 0;
  show_value(&slc);
  show_op("value of array is now:");
  show_value(&arr);
  println!();

  /*-- tuple --*/
  show_label("tuples");
  show_op("let tpl = (42, 'z', \"abc\", 3.14159)");
  #[allow(clippy::approx_constant)]
  let tpl = (42, 'z', "abc", 3.14159);
  show_type(&tpl);
  show_value(&tpl);
  show_op("value of second element is:");
  show_value(&tpl.1);
  println!();

  /*-- string --*/
  show_label("strings");
  show_op("let s = \"a string\".to_string()");
  let mut s = "a string".to_string();
  show_type(&s);
  show_value(&s);
  show_op("s.push_str(\" plus more\")");
  s.push_str(" plus more");
  show_value(&s);
  println!();

  /*-- reference --*/
  show_label("references");
  show_op("let r = &s");
  let r = &s;
  show_type(&r);
  show_value(&r);
  println!();

  /*-- struct --*/
  show_label("structures");
  #[derive(Debug)]
  struct DemStr { i:i32, c:char, d:f64, }
  show_op("let st = DemStr { i:1, c:'a', d:0.333 }");
  let st = DemStr { i:1, c:'a', d:0.333 };
  show_type(&st);
  show_value(&st);
  let second = st.c;
  show_op("let second = st.c");
  show_value(&second);
  println!();

  /*-- enum --*/
  show_label("enumerations");

  #[derive(Debug)]
  enum LangAge { Recent, Ancient }
  
  #[derive(Debug)]
  enum Langs {
    Rust(LangAge), Fortran(LangAge)
  }
  
  let a_lang = Langs::Rust(LangAge::Recent);
  show_type(&a_lang);
  show_value(&a_lang);
  
  let old_lang = Langs::Fortran(LangAge::Ancient);
  show_type(&old_lang);
  show_value(&old_lang);

  /*-- matching requires handling all branches --*/
  match a_lang {
    Langs::Rust(LangAge::Recent) => { println!("  Rust is recent"); }
    Langs::Rust(LangAge::Ancient) => { println!("  Rust is ancient"); }
    Langs::Fortran(LangAge::Recent) => { println!("  Fortran is recent"); }
    Langs::Fortran(LangAge::Ancient) => { println!("  Fortran is ancient"); }
  }
  /*-------------------------------------------------------
    if let can examine one branch and provide 
    blanket handling for others 
  */
  if let Langs::Rust(LangAge::Recent) = a_lang {
    println!("  Rust was stablized in 2015")
  } else {
    println!("  this language isn't very interesting");
  }
}

fn move_copy() {
  show_title("Demonstrate Copy and Move");

  show_label("copy array of integers");
  show_op("let arri = [ 1, 2, 3, 2, 1]");
  let arri = [ 1, 2, 3, 2, 1];
  show_value(&arri);
  show_op("let carri = arri");
  let carri = arri;
  show_value(&carri);
  // the next statement succeeds because arri was copied 
  // println!("{arri:?}");
  println!();

  show_label("copy array of &strs");
  show_op("let arri = [ \"1\", \"2\", \"3\", \"2\", \"1\"]");
  let arri = [ "1", "2", "3", "2", "1"];
  show_value(&arri);
  show_op("let carri = arri");
  let carri = arri;
  show_value(&carri);
  // the next statement succeeds because arri was copied 
  // println!("{arri:?}");
  println!();

  show_label("move array of Strings");
  show_op(
    "let arri = [\"1\".to_owned(), \"2\".to_owned(), 
    \"3\".to_owned(), \"2\".to_owned(), \"1\".to_owned()])"
  );  
  /*------------------------------------------------------
    to_owned() converts copy type &str 
    to move type String
  */
  let arri = [ 
    "1".to_owned(), "2".to_owned(), "3".to_owned(), 
    "2".to_owned(), "1".to_owned()
  ];
  show_value(&arri);
  show_op("let carri = arri");
  let carri = arri;
  show_value(&carri);
  // the next statement fails because arri was moved 
  // println!("{arri:?}");
  println!("  arri moved so no longer valid\n");
  println!("  an aggregate of all copy types is copy");
  println!("  an aggregate with at least one move type element is move");

  
}

fn main() {
  basic_aggr();
  move_copy();
  println!("\n  That's all Folks!");
}
