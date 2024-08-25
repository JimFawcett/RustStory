///////////////////////////////////////////////////////////
// Demonstrate polymorphic operations using Rust Traits  //
// user_def::main.rs                                     //
///////////////////////////////////////////////////////////

mod userdef;
use userdef::*;

#[allow(dead_code)]
pub fn run() {

  print!(
    "\n\n  {}",
    "-- demo polymorphic struct instances --\n"
  );
  let presenter : Presenter = Presenter;
  let joe : Friend = Friend::new("Joe".to_string());
  let sue : Friend = Friend::new("Sue".to_string());
  let team_lead : TeamLead = TeamLead;

  let mut v :Vec<&dyn Speaker> = Vec::new();
  v.push(&presenter);
  v.push(&joe);
  v.push(&sue);
  v.push(&team_lead);

  /*
    Presenters, Friends, and TeamLeads are all Speakers, i.e.,
    they implement the Speaker trait, so they all can be
    treated uniformily, as speakers.
   */
  for speaker in v.iter() {
    print!("\n  {:?}",speaker.salutation());
  }
}

fn main() {
  run();
}
