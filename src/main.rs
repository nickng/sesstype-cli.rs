extern crate clap;
extern crate sesstype;

use clap::{Arg, App, SubCommand};
use std::error::Error;
use std::fs::File;
use std::io::stdin;
use std::io::Read;
use std::io::prelude::Write;
use std::path::Path;

fn read_input_file(input_file: &str) -> String {
    if input_file == "-" {
        let stdin = stdin();
        let mut file = stdin.lock();
        let mut content = String::new();
        file.read_to_string(&mut content).expect(
            "Error reading from stdin",
        );
        content
    } else {
        let mut file = File::open(input_file).expect("File not found");
        let mut content = String::new();
        file.read_to_string(&mut content).expect(
            "Error reading the file",
        );
        content
    }
}

fn write_output_file(output: String, output_file: Option<&str>) {
    match output_file {
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

fn main() {
    let matches = App::new("sesstype command line interface")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A command-line interface to the sesstype mini language")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Output to FILE")
                .global(true)
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("parse")
                .about("Parse a sesstype file")
                .arg(
                    Arg::with_name("input.mpst")
                        .help("Input sesstype files (use - to read from stdin)")
                        .multiple(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("global")
                        .short("g")
                        .long("global")
                        .help("Parse as a global types (default)")
                        .conflicts_with("local"),
                )
                .arg(
                    Arg::with_name("local")
                        .short("l")
                        .long("local")
                        .help("Parse as a local types")
                        .conflicts_with("global"),
                ),
        )
        .subcommand(
            SubCommand::with_name("project")
                .about("Perform endpoint projection on the given session type")
                .arg(
                    Arg::with_name("input.mpst")
                        .help("Input sesstype files (use - to read from stdin)")
                        .multiple(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("role")
                        .short("r")
                        .long("role")
                        .value_name("ROLE")
                        .help("Name of role to project for")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    // Handle parse subcommand.
    //
    // Parse subcommand takes input.mpst and parses them one by one
    // By default the files are parsed as global types.
    // --local option switches to parsing as local types.
    //
    if let Some(parse) = matches.subcommand_matches("parse") {
        if let Some(inputs) = parse.values_of("input.mpst") {
            for input in inputs {
                let content = read_input_file(input);
                let output =
                    if parse.is_present("local") {
                        // Parse local type.
                        let (local, _registry) = sesstype::parser::parse_local_type(content)
                            .expect("Cannot parse local type");
                        local.to_string()
                    } else {
                        // Parse global type.
                        let (global, _registry) = sesstype::parser::parse_global_type(content)
                            .expect("Cannot parse global type");
                        global.to_string()
                    };
                write_output_file(output, parse.value_of("output"));
            }
        }
    } else
    // Handle project subcommand.
    //
    // Parse input.mpst as global type and project with respect to R
    // supplied by --role R
    //
    if let Some(project) = matches.subcommand_matches("project") {
        if let Some(role) = project.value_of("role") {
            if let Some(inputs) = project.values_of("input.mpst") {
                for input in inputs {
                    let content = read_input_file(input);
                    let (global, registry) = sesstype::parser::parse_global_type(content).expect(
                        "Cannot parse global type",
                    );
                    let role = match registry.find_role_str(role) {
                        Some(role) => role,
                        None => panic!("Cannot project global type: role {} not found", role),
                    };
                    let projected = match sesstype::project(&global, &role) {
                        Some(local) => local.to_string(),
                        None => String::from("(empty)"),
                    };
                    write_output_file(projected, project.value_of("output"));
                }
            }
        }
    } else {
        println!(
            "no subcommand specified, use -h to see detailed usage\n{}",
            matches.usage()
        )
    }
}
