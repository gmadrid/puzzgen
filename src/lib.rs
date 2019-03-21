pub use puzz::Puzzle;

#[macro_use]
mod geom;
mod puzz;

#[macro_use]
extern crate error_chain;

#[allow(deprecated)]
pub mod errors {
    error_chain! {
        foreign_links{
            ParseFloat(std::num::ParseFloatError);
            ParseInt(std::num::ParseIntError);
            Clap(clap::Error);
        }
    }
}
