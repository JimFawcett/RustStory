/////////////////////////////////////////////////////////////
// main.rs - executes named module                         //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 24 Feb 2020  //
/////////////////////////////////////////////////////////////

mod probe_struct;
mod probe_traits;
mod types;

fn main() {

    probe_struct::run();
    //probe_traits::run();
    //types::run();
    print!("\n\n");
}
