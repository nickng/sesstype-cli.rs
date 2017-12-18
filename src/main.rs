extern crate getopts;
extern crate sesstype;

use getopts::Options;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::stdin;
use std::io::Read;
use std::io::prelude::Write;
use std::path::Path;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "output", "Output to FILE", "FILE");
    opts.optopt("p", "project", "Endpoint projection", "ROLE");
    opts.optflag("l", "local", "Parse as local types");
    opts.optflag("h", "help", "Display this message");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let outfile = matches.opt_str("o");
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let s = if input == "-" {
        let stdin = stdin();
        let mut file = stdin.lock();
        let mut content = String::new();
        file.read_to_string(&mut content).expect(
            "Error reading from stdin",
        );
        content
    } else {
        let mut file = File::open(input).expect("File not found");
        let mut content = String::new();
        file.read_to_string(&mut content).expect(
            "Error reading the file",
        );
        content
    };
    let read = String::from(s);

    let output = if matches.opt_present("l") {
        // Parse local type.
        let (local, _registry) =
            sesstype::parser::parse_local_type(read).expect("Cannot parse local type");
        local.to_string()

    } else {
        // Parse global type.
        let (global, registry) =
            sesstype::parser::parse_global_type(read).expect("Cannot parse global type");

        let proj_role = matches.opt_str("p");
        match proj_role {
            // If projecting, return projected local type.
            Some(proj_role) => {
                let role = match registry.find_role(proj_role.clone()) {
                    Some(role) => role,
                    None => panic!("Cannot project global type: role {} not found", &proj_role),
                };
                match sesstype::project(&global, &role) {
                    Some(local) => local.to_string(),
                    None => String::from("(empty)"),
                }
            }
            // Return global type otherwise.
            None => global.to_string(),
        }
    };


    match outfile {
        Some(outfile) => {
            let path = Path::new(&outfile);
            if path.exists() {
                panic!("Cannot write to output: file exists");
            } else {
                let mut file = match File::create(&path) {
                    Err(why) => {
                        panic!(
                            "Cannot write to output {}: {}",
                            path.display(),
                            why.description()
                        )
                    }
                    Ok(file) => file,
                };
                match file.write_all(output.as_bytes()) {
                    Err(why) => {
                        panic!(
                            "Cannot write to output {}: {}",
                            path.display(),
                            why.description()
                        )
                    }
                    Ok(_) => println!("Output written to {}", path.display()),
                }
            }
        }
        None => println!("{}", output),
    }
}
