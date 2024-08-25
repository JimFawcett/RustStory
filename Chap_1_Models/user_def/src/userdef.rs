///////////////////////////////////////////////////////////
// user-defined traits act like interfaces               //
// userdef.rs                                            //
///////////////////////////////////////////////////////////

pub trait Speaker {
  fn salutation(&self) -> String;
}

///////////////////////////////////////////////////////////
// The following structs act like classes that implement
// a Speaker interface

#[derive(Debug,Copy,Clone)]
pub struct Presenter;
impl Speaker for Presenter {
  fn salutation(&self) -> String {
    "Presenter: Hello, today we will discuss ...".to_string()
  }
}

#[derive(Debug)]
pub struct Friend {
  pub name : String,
}
impl Speaker for Friend {
  fn salutation(&self) -> String {
    let mut s = String::from("Friend:    Hi good buddy, its me, ");
    let nm = self.name.as_str();
    s.push_str(nm);
    return s;
  }
}
impl Friend {
  #[allow(dead_code)]
  pub fn new(name : String) -> Self {
    Self {
      name,
    }  // note: no semicolon so Self is returned
  }
}
#[derive(Debug,Copy,Clone)]
pub struct TeamLead;
impl Speaker for TeamLead {
  fn salutation(&self) -> String {
    "TeamLead:  Hi, I have a task for you ...".to_string()
  }
}

