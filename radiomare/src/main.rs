//! Main [RadioMare](https://codeberg.org/r6915ee/radiomare/) client.
//!
//! **RadioMare** is a mania rhythm game that primarily leverages
//! [Rust](https://rust-lang.org/) and [Bevy](https://bevy.org/) to provide
//! a compatible program.
//!
//! This crate in specific is the main client, for which all libraries
//! stored in the monorepo are built for.
//!
//! # Usage
//!
//! This client is a complete, executable binary. There is no need to use it as
//! a library, since its setup doesn't allow this behavior.

use bevy::prelude::*;
use crashlog::cargo_metadata;
use log::info;

/// Entry point for the program.
fn main() {
    let mut crash_metadata: crashlog::ProgramMetadata = cargo_metadata!();
    crash_metadata.package = std::borrow::Cow::Borrowed("FunkSystem");
    crashlog::setup!(
        cargo_metadata!().capitalized(),
        false,
        "\
        {package} has crashed.\n\nPlease open an issue on the issue tracker hosted \
        at {repository}, and copy and paste the contents of {log_path} where \
        appropriate.
    "
    );

    clang_log::init(log::Level::Warn, "radiomare");
    info!("initializing game");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RadioMare".into(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
