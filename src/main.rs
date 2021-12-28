#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

/*!
# Proton Call

Run any Windows program through [Valve's Proton](https://github.com/ValveSoftware/Proton).

## Usage:

Defaults to the latest version of Proton.
```
proton-call -r foo.exe
```

Defaults to the latest version of Proton, all extra arguments passed to the executable.
```
proton-call -r foo.exe --goes --to program
```

`--goes --to program` are passed to the proton / the program

Uses specified version of Proton, any extra arguments will be passed to the executable.
```
proton-call -p 5.13 -r foo.exe
```

Uses custom version of Proton, give the past to directory, not the Proton executable itself.
```
proton-call -c '/path/to/Proton version' -r foo.exe
```
 */

extern crate jargon_args;
extern crate lliw;

use proton_call::error::{Error, Kind};
use proton_call::{pass, throw, Config, Index, Proton, Version};
use std::path::PathBuf;
use std::process::exit;

/// Type to handle and parse command line arguments with `Jargon`
#[derive(Debug)]
struct Args {
    program: PathBuf,
    version: Version,
    log: bool,
    custom: Option<PathBuf>,
    args: Vec<String>,
}

/// Main function which purely handles errors
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program: String = args[0].split('/').last().unwrap_or(&args[0]).to_string();
    if let Err(e) = proton_caller(args) {
        eprintln!("{}: {}", program, e);
        let code = e.kind() as i32;
        exit(code);
    }
}

/// Effective main function which parses arguments
fn proton_caller(args: Vec<String>) -> Result<(), Error> {
    use jargon_args::Jargon;

    // args.insert(args.len(), "--index".to_string());

    let mut parser: Jargon = Jargon::from_vec(args);

    if parser.contains(["-h", "--help"]) {
        help();
    } else if parser.contains(["-v", "--version"]) {
        version();
    } else if parser.contains(["-i", "--index"]) {
        let config: Config = Config::open()?;
        let common_index = Index::new(&config.common())?;
        println!("{}", common_index);
    } else {
        let config: Config = Config::open()?;
        let args = Args {
            program: parser.result_arg(["-r", "--run"])?,
            version: parser.option_arg(["-p", "--proton"]).unwrap_or_default(),
            log: parser.contains(["-l", "--log"]),
            custom: parser.option_arg(["-c", "--custom"]),
            args: parser.finish(),
        };

        let proton = if args.custom.is_some() {
            custom_mode(&config, args)?
        } else {
            normal_mode(&config, args)?
        };

        let exit = proton.run()?;

        if !exit.success() {
            if let Some(code) = exit.code() {
                throw!(Kind::ProtonExit, "code: {}", code);
            }
            throw!(Kind::ProtonExit, "an error");
        }
    }

    Ok(())
}

fn get_proton_path(index: &mut Index, version: Version) -> Result<PathBuf, Error> {
    if let Some(path) = index.get(&version) {
        return Ok(path);
    }
    eprintln!("{}info:{} Proton {} not found, reindexing...", lliw::Fg::Blue, lliw::Reset, version);
    index.index()?;
    index.get(&version).ok_or_else(|| Error::new(Kind::ProtonMissing, format!("Proton {} does not exist", version)))
}

/// Runs caller in normal mode, running indexed Proton versions
fn normal_mode(config: &Config, args: Args) -> Result<Proton, Error> {
    let mut index: Index = Index::new(&config.common())?;

    let proton_path: PathBuf = get_proton_path(&mut index, args.version)?;

    let proton: Proton = Proton::new(
        args.version,
        proton_path,
        args.program,
        args.args,
        args.log,
        config.data(),
        config.steam(),
    );

    pass!(proton)
}

/// Runs caller in custom mode, using a custom Proton path
fn custom_mode(config: &Config, args: Args) -> Result<Proton, Error> {
    if let Some(custom) = args.custom {
        let proton: Proton = Proton::new(
            Version::from_custom(custom.as_path()),
            custom,
            args.program,
            args.args,
            args.log,
            config.data(),
            config.steam(),
        );

        return pass!(proton);
    }

    throw!(Kind::Internal, "failed to run custom mode")
}

#[doc(hidden)]
static HELP: &str = "\
Usage: proton-call [OPTIONS]... EXE [EXTRA]...

Options:
    -c, --custom [PATH]     Path to a directory containing Proton to use
    -h, --help              View this help message
    -i, --index             View an index of installed Proton versions
    -l, --log               Pass PROTON_LOG variable to Proton
    -p, --proton [VERSION]  Use Proton VERSION from `common`
    -r, --run EXE           Run EXE in proton
    -v, --version           View version information

Config:
    The config file should be located at '$XDG_CONFIG_HOME/proton.conf' or '$HOME/.config/proton.conf'
    The config requires two values.
    Data: a location to any directory to contain Proton's runtime files.
    Steam: the directory to where steam is installed (the one which contains the steamapps directory).
    Common: the directory to where your proton versions are stored, usually Steam's steamapps/common directory.
    Example:
        data = \"/home/avery/Documents/Proton/env/\"
        steam = \"/home/avery/.steam/steam/\"
        common = \"/home/avery/.steam/steam/steamapps/common/\"
";

#[doc(hidden)]
fn help() {
    println!("{}", HELP);
}

#[doc(hidden)]
fn version() {
    println!(
        "Proton Caller (proton-call) {} Copyright (C) 2021 {}",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS")
    );
}
