extern crate rc;
#[macro_use] extern crate clap;

use rc::errors::*;

use std::io::{self, BufRead, BufReader};
use std::fs::File;

use clap::{Arg, App, ArgMatches};


pub fn main() {
    let matches = App::new("rc")
        .version(crate_version!())
        .arg(Arg::with_name("eval")
             .long("eval")
             .short("e")
             .takes_value(true)
             .help("Evaluate a given string"))
        .arg(Arg::with_name("file")
             .long("file")
             .short("f")
             .takes_value(true)
             .help("Specify file to read"))
        .get_matches();

    if let Err(ref e) = run(matches) {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let stderr_msg = "Error writing to stderr";
        writeln!(stderr, "error: {}", e).expect(stderr_msg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(stderr_msg);
        }

        // `RUST_BACKTRACE=1`
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(stderr_msg);
        }

        ::std::process::exit(1);
    }
}


fn run(matches: ArgMatches) -> Result<()> {
    if let Some(input) = matches.value_of("eval") {
        rc::Env::new().eval(input).chain_err(|| "Evaluation error")?;
        return Ok(())
    }
    if let Some(file) = matches.value_of("file") {
        let mut env = rc::Env::new();
        let file = File::open(file)
            .chain_err(|| format!("Unable to open file: {}", file))?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            env.eval(&line).chain_err(|| "Evaluation error")?;
        }
        return Ok(())
    }

    let mut env = rc::Env::new();
    let stdin = io::stdin();
    let std_buf = stdin.lock();
    for line in std_buf.lines() {
        let line = line.expect("Failed to read line");
        env.eval(&line).chain_err(|| "Evaluation error")?;
    }
    Ok(())
}
