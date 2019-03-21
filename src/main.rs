#[macro_use]
extern crate clap;

use clap::{AppSettings, Arg, ArgMatches};
use puzzgen::errors::*;
use puzzgen::Puzzle;
use std::env;
use std::ffi::OsString;
use std::str::FromStr;

fn real_main() -> Result<()> {
    let args = Args::parse()?;

    let puzzle = Puzzle::builder()
        .size(args.width()?, args.height()?)
        .pieces(args.columns()?, args.rows()?)
        .vertex_jitter_pct(args.vertex_jitter()?)
        .build();

    // unwrap: making gross assumptions.
    let svg = puzzle.to_svg().unwrap();

    println!("{}", svg);
    Ok(())
}

struct Args<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> Args<'a> {
    fn parse() -> Result<Args<'a>> {
        Ok(Args {
            matches: parse_from(env::args_os())?,
        })
    }

    pub fn rows(&self) -> Result<usize> {
        // unwrap: safe because there is a default value.
        usize::from_str(&self.matches.value_of_lossy("rows").unwrap()).map_err(Error::from)
    }

    pub fn columns(&self) -> Result<usize> {
        // unwrap: safe because there is a default value.
        usize::from_str(&self.matches.value_of_lossy("columns").unwrap()).map_err(Error::from)
    }

    pub fn height(&self) -> Result<f32> {
        // unwrap: safe because there is a default value.
        f32::from_str(&self.matches.value_of_lossy("height").unwrap()).map_err(Error::from)
    }

    pub fn width(&self) -> Result<f32> {
        // unwrap: safe because there is a default value.
        f32::from_str(&self.matches.value_of_lossy("width").unwrap()).map_err(Error::from)
    }

    pub fn vertex_jitter(&self) -> Result<f32> {
        // unwrap: safe because there is a default value.
        f32::from_str(&self.matches.value_of_lossy("vertex_jitter").unwrap()).map_err(Error::from)
    }
}

fn main() {
    env_logger::init();

    match real_main() {
        Ok(_) => {}
        Err(err) => {
            match err {
                // Clap gets special attention. ('-h' for example is better handled by clap.)
                Error(ErrorKind::Clap(ce), _) => ce.exit(),
                _ => println!("Error: {}", err),
            }
        }
    }
}

fn parse_from<'a, I, T>(itr: I) -> Result<ArgMatches<'a>>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    app_from_crate!()
        .setting(AppSettings::StrictUtf8)
        // Arguments
        .arg(
            Arg::with_name("vertex_jitter")
                .long("vertex_jitter")
                .help("Percent of piece size to jitter vertices.")
                .value_name("jitter")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("width")
                .long("width")
                .short("w")
                .help("The puzzle width (in mm)")
                .default_value("300"),
        )
        .arg(
            Arg::with_name("height")
                .long("height")
                .short("h")
                .help("The puzzle height (in mm)")
                .default_value("200"),
        )
        .arg(
            Arg::with_name("rows")
                .long("rows")
                .short("r")
                .help("The number of pieces from top to bottom")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("columns")
                .long("cols")
                .short("c")
                .help("The number of pieces across")
                .default_value("15"),
        )
        .get_matches_from_safe(itr)
        .map_err(Error::from)
}
