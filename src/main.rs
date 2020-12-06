#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate prettytable;

use std::process;

mod cli;
mod permissions;
mod utils;

fn main() {
    // verify unix
    if !cfg!(unix) {
        eprintln!("Not a unix system, exiting...");
        process::exit(-1);
    }

    let action;
    let matches = cli::create_cli_options().get_matches();

    //Todo: Would probably be smoother to split the print and calc to different bins
    //      and use the 'permissions' as a lib instead... Maybe in the future

    if let Some(sub_m) = matches.subcommand_matches("print") {
        action = match permissions::handle_print(sub_m) {
            Ok(result) => Ok(result),
            // first part of error message
            Err(cmd_err) => {
                eprint!("ERROR - Handle print command failed with: ");
                Err(cmd_err)
            }
        };

        // second part of error message + exit failure
        if let Err(e) = action {
            eprintln!("{}", e);
            process::exit(-1)
        }

        // Sub cmd successfully executed
        process::exit(0);
    }

    if let Some(sub_m) = matches.subcommand_matches("calculate") {
        action = match permissions::handle_calculate(sub_m) {
            Ok(result) => Ok(result),
            // first part of error message
            Err(cmd_err) => {
                eprint!("ERROR - Handle calculate command failed with: ");
                Err(cmd_err)
            }
        };

        // second part of error message + exit failure
        if let Err(e) = action {
            eprintln!("{}", e);
            process::exit(-1)
        }

        // Sub cmd successfully executed
        process::exit(0);
    }
}
