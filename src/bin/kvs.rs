// Import necessary components from the `clap` crate to define CLI structure.
use clap::{App, AppSettings, Arg, SubCommand};

// Import `exit` to manually terminate the program with a status code.
use std::process::exit;

fn main() {
    // Create a new CLI application using metadata from Cargo.toml.
    let matches = App::new(env!("CARGO_PKG_NAME")) // Set the binary name from package metadata.
        .version(env!("CARGO_PKG_VERSION")) // Set the version from package metadata.
        .author(env!("CARGO_PKG_AUTHORS")) // Set the author(s) from package metadata.
        .about(env!("CARGO_PKG_DESCRIPTION")) // Set the description from package metadata.
        // Disables the auto-generated "help" subcommand (we'll just use -h/--help flags).
        .setting(AppSettings::DisableHelpSubcommand)
        // Makes it mandatory to provide a subcommand (otherwise shows help).
        .setting(AppSettings::SubcommandRequiredElseHelp)
        // Allows using subcommands without requiring `--version` to be repeated.
        .setting(AppSettings::VersionlessSubcommands)
        // Define the `set` subcommand: used to store a key-value pair.
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("A string key").required(true)) // First argument: key
                .arg(
                    Arg::with_name("VALUE")
                        .help("The string value of the key")
                        .required(true), // Second argument: value
                ),
        )
        // Define the `get` subcommand: used to retrieve the value of a key.
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)), // Argument: key
        )
        // Define the `rm` subcommand: used to delete a key-value pair.
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)), // Argument: key
        )
        // Parse and retrieve the CLI arguments from the command line.
        .get_matches();

    // Match on which subcommand was used by the user
    match matches.subcommand() {
        ("set", Some(_matches)) => {
            // `set` logic is not yet implemented — show error and exit.
            eprintln!("unimplemented");
            exit(1);
        }
        ("get", Some(_matches)) => {
            // `get` logic is not yet implemented — show error and exit.
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_matches)) => {
            // `rm` logic is not yet implemented — show error and exit.
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(), // This should never happen due to `SubcommandRequiredElseHelp`
    }
}
