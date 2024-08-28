/////////////////////////////////////////////////////////////
// string_probes::main.rs - basic string operations        //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 25 Feb 2020  //
/////////////////////////////////////////////////////////////

#[allow(unused_imports)]
use display::{ 
  log, slog, show, show_type, show_value, 
  putline, putlinen, main_title, sub_title 
};
#[allow(unused_imports)]
use std::fmt::{ Debug, Display };

///////////////////////////////
// for later
// #[allow(unused_imports)]
// use std::ffi::OsString;

fn show_op(s:&str) {
  let strg = "--- ".to_owned() + s + " ---";
  print!("{}", strg);
}

fn put<T>(t:T) where T:Display {
  print!("{}", t);
}

fn putdb<T>(t:T) where T:Debug {
  print!("{:?}", t);
}

fn put_str(s:&String) {
  print!("{}",s);
}
/*-----------------------------------------------------------
   Note:
   Strings hold utf8 characters, which vary in size, so you
   you can't directly index String instances.
*/
#[allow(dead_code)]
pub fn at(s:&String, i:usize) -> char {
  s.chars().nth(i).unwrap()
}
/*-----------------------------------------------------------
   note: 
   - order n, as str chars are utf8, e.g., from 1 to 5 bytes
   - this ugliness is one way to index
   - see below for another, not much better way
*/
#[allow(dead_code)]
pub fn vectorize(s: &str) -> Vec<char> {
  s.chars().collect::<Vec<char>>()
}
/*-- note: order n, from vectorize -- prefer at, above --*/
#[allow(dead_code)]
pub fn get_char(s:&str, i:usize) -> char {
    vectorize(s)[i]
}
/*-- stringize - order n --*/
#[allow(dead_code)]
pub fn stringize(v: &Vec<char>) -> String {
  return v.into_iter().collect()
}

fn main() {

    main_title("string_probes");
    putlinen(2);

    /*-- char --*/

    show_op("let v = vec!['R', 'u', 's', 't']");
    let v:Vec<char> = vec!['R', 'u', 's', 't'];
    log(&v);
    log(&'R');
    putlinen(2);

    show_op("let ch = 'a' as u8");
    let ch:u8 = 'a' as u8;
    log(&ch);
    show("char is ", &(ch as char));
    putlinen(2);

    /*-- String --*/

    show_op("let s = String::from(\"Rust\")");
    let s:String = String::from("Rust");
    log(&s);
    let i:usize = 2;
    let ch = at(&s, i);
    print!("\n  in string \"{}\", char at {} is {}", &s, i, ch);
    show("length in bytes of s = {:?}", &s.len());
    putlinen(2);

    show_op("let v = Vec::from(s.clone())");
    let s1 = s.clone();
    let v:Vec<u8> = Vec::from(s1);
    log(&v[0]);
    show("vec from string",&v);
    putlinen(2);

    /*----------------------------------------------------- 
      Displaying emoji's to illustrate the potential
      of using utf-8.
    */
    show_op("displaying emoji's");
    let mut s2 = String::new();
    s2.push_str("\u{1F600}");
    s2.push('\u{1F601}');
    s2.push('\u{1F602}');
    s2.push('\u{1F609}');
    print!("\n  {}", s2);
    print!("\n  {}", '\u{1F601}');
    putlinen(2);

    /*-- str --*/

    show_op("let s_slice = &s[..]");
    let s_slice = &s[..];   // slice containing all chars of s
    log(&s_slice);
    show("s_slice = ", &s_slice);
    putlinen(2);

    show_op("let s_slice2 = s.as_str()");
    let s_slice2 = s.as_str();
    log(&s_slice2);
    putlinen(2);

    /*-- create string and mutate --*/

    show_op("let mut s = string::new()");
    let mut s = String::new();
    s.push('a');
    s.push(' ');
    s.push_str("test string");
    log(&s);
    putlinen(2);

    show_op("let t = s.replace(from: \"string\", to: \"Rust String\"");
    let t = s.replace("string","Rust String");
    log(&t);
    putlinen(2);

    show_op("tok in s.split_whitespace()");
    for tok in s.split_whitespace() {
      print!("\n  {}", tok);
    }
    putline();

    /*-----------------------------------------------------
       Another, order n, way to index string:
      - chars returns iterator over utf8 chars in string slice
      - nth(i) calls next on iterator until it gets to i
      - nth(i) returns std::option::Option<char>:
         - that contains Some(ch) or None if operation failed
    */
    show("\n  s = ", &s);
    putline();
    show_op("let result = s.chars().nth(0)");
    putline();
    let result = s.chars().nth(0);
    match result {
      Some(r) => show("  s.chars().nth(0) = ", &r),
      None => print!("\n  couldn't extract char"),
    }
    putline();
    show_op("let result = s.chars().nth(2)");
    putline();
    let result = s.chars().nth(2);
    match result {
      Some(r) => show("  s.chars().nth(2) = ", &r),
      None => print!("\n  couldn't extract char"),
    }
    putlinen(2);

    {
        /*-------------------------------------------------
           Caution here: 
           - slice is returning array of bytes, not utf8 chars
           - this works only because we use all ASCII chars
        */
        /*-- slices are non-owning views and are borrows of s --*/
        show_op("let slice_all = &s");
        let slice_all = &s;
        log(&slice_all);
        show("slice_all = ", &slice_all);
        putlinen(2);

        show_op("let third = &s[2..3]");
        let third = &s[2..3];       // string slice with one char
        log(&third);
        show("\n  third = ",&third);
        putlinen(2);

        /*-- this works for utf-8 encoding --*/
        show_op("let ch = third.chars().nth(0)");
        let ch = third.chars().nth(0);  // 
        log(&ch);
        match ch {
          Some(x) => { log(&x); show("\n  match ch = ", &x); },
          None => print!("\n can't return ch"),
        }
        
        ///////////////////////////////////////////////////
        // compile fails 
        // - can't modify owner while borrows are active
        //------------------------------------------------
        // s.push('Z');
        // log(&slice_all);

    }   // elem borrow ends here

    s.push('Z');  // ok, borrows no longer active
    putlinen(2);

    /* format_args! macro */

    show_op("let s = std::fmt::format(format_args!(...))");
    let s = std::fmt::format(format_args!("\n  {}, {}, {}", 1, 2, 3.5));
    put_str(&s);
    put(&s);
    putlinen(2);

    show_op("struct S { x:i32, y:f64, s:String, }");
    #[allow(dead_code)]
    #[derive(Debug)]
    struct S {x:i32, y:f64, s:String, }
    let st:S = S { x:3, y:4.2, s:"xyz".to_string() };
    put("\n  ");
    putdb(&st);
    putline();

    sub_title("That's all Folks!");
    putlinen(2);
}
