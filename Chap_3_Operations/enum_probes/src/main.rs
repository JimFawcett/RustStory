// enum_probes::main.rs

use display::{*};
use std::fmt::{Debug};

#[allow(dead_code)]
#[derive(Debug)]
enum Name { John, Jim=42, Jack }

#[allow(dead_code)]
#[derive(Debug)]
enum NameTuple { 
    John(String, u32), Jim(String, u32), Jack(String, u32) 
}

#[allow(dead_code)]
#[derive(Debug)]
enum NameStruct {
    John { occup:String, id:u32 },
    Jim  { occup:String, id:u32 },
    Jack { occup:String, id:u32 }
}
fn main() {

    main_title("Demonstrating enum_probes");
    print!("\n  - enumerations, match, if let");
    putline();

    /*-- enum discriminant --*/
    sub_title("  -- enum discriminant --  ");
    let test = Name::Jim;
    match test {
        Name::John => {
            let john_discriminant = Name::John as u32;
            print!(
            "\n  I am John. my discriminant is {:?}", 
            john_discriminant
        )},
        Name::Jim => {
            let jim_discriminant = Name::Jim as u32;
            print!(
            "\n  I am Jim. my discriminant is {:?}", 
            jim_discriminant
        )},
        Name::Jack => {
            let jack_discriminant = Name::Jack as u32;
            print!(
            "\n  I am Jack. my discriminant is {:?}", 
            jack_discriminant
        )},
    }
    putline();

    let test1 = Name::John;
    let test2 = Name::Jim;
    let test3 = Name::Jack;

    if let Name::Jack = test1 {
        print!("\n  I am John");
    }
    else {
        print!("\n  I am not John");
    }
    if let Name::Jack = test2 {
        print!("\n  I am Jim");
    }
    else {
        print!("\n  I am not Jim");
    }
    if let Name::Jack = test3 {
        print!("\n  I am Jack");
    }
    else {
        print!("\n  I am not Jack");
    }
    putline();

    /*-- enum tuple --*/
    sub_title("  -- enum tuple --  ");
    let value = NameTuple::John("pilot".to_string(), 52);
    if let NameTuple::John(occup, id) = value {
        print!(
            "
  my name is John
  occupupation is {}
  id is {}", occup, id
        );
    }
    putline();

    /*-- enum struct --*/
    sub_title("  -- enum struct --  ");
    let value = NameStruct::Jack { occup:"plumber".to_string(), id:32 };
    match value {
        NameStruct::Jack {occup, id} => print!("\n Jack - occup: {}, id: {}", occup, id),
        _ => print!("\n  not Jack")
    }
    putline();

    println!("\n\nThat's all Folks!\n");
}
