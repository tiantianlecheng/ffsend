extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate ffsend_api;

mod action;
mod app;
mod cmd;
mod error;
mod progress;
mod util;

use action::download::Download;
use action::upload::Upload;
use cmd::Handler;
use error::Error;
use util::quit_error;

/// Application entrypoint.
fn main() {
    // Parse CLI arguments
    let cmd_handler = Handler::parse();

    // Invoke the proper action
    if let Err(err) = invoke_action(&cmd_handler) {
        quit_error(err);
    };
}

/// Invoke the proper action based on the CLI input.
///
/// If no proper action is selected, the program will quit with an error
/// message.
fn invoke_action(handler: &Handler) -> Result<(), Error> {
    // Match the upload command
    if let Some(cmd) = handler.upload() {
        return Upload::new(&cmd).invoke()
            .map_err(|err| err.into());
    }

    // Match the download command
    if let Some(cmd) = handler.download() {
        return Download::new(&cmd).invoke()
            .map_err(|err| err.into());
    }

    // No subcommand was selected, show general help
    Handler::build()
        .print_help()
        .expect("failed to print command help");

    Ok(())
}
