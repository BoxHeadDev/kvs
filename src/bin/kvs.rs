// Import necessary components from the clap crate to build the CLI interface.
use clap::{App, AppSettings, Arg, SubCommand};

// Import exit function to terminate the program with a status code.
use std::process::exit;

fn main() {
    // Create the CLI application with metadata from Cargo.toml.
    // Set application name, version, author, and description.
    // Apply settings to customize behavior:
    // - Disable help subcommand.
    // - Require a subcommand to be provided.
    // - Allow version flag without requiring it on subcommands.

    // Define the `set` subcommand:
    // - Requires a KEY and VALUE.
    // - Describes its purpose: store a string value under a string key.

    // Define the `get` subcommand:
    // - Requires a KEY.
    // - Describes its purpose: retrieve a value for the given key.

    // Define the `rm` subcommand:
    // - Requires a KEY.
    // - Describes its purpose: remove the key-value pair if it exists.

    // Parse CLI arguments and match on the provided subcommand.
    // Handle each subcommand by implementing corresponding logic:
    // - For `set`, store the key-value pair.
    // - For `get`, retrieve and print the value or show "Key not found".
    // - For `rm`, remove the key or show "Key not found".
    // - Unreachable arm included for exhaustiveness due to clap settings.
}
