/////////////////////////////////////////////////////////////
// main.rs - executes named module                         //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 24 Feb 2020  //
/////////////////////////////////////////////////////////////

mod probe_struct;
mod probe_traits;
mod types;
#[allow_]
enum Run {
  Structs,
  Traits,
  Types,
}

fn main() {

  let run = Run::Structs;

  match run {
    Run::Structs => probe_struct::run(),
    Run::Traits => probe_traits::run(),
    Run::Types => types::run(),
  }
    print!("\n\n");
}
