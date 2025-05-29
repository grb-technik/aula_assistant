// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::{arg, command, crate_name, crate_version, ArgAction};

fn main() {
    let matches = command!()
        // help
        .next_line_help(false)
        // version
        .disable_version_flag(true)
        .arg(arg!(-v --version "Print version").action(ArgAction::SetTrue))
        // license
        .arg(arg!(--license "Print license information").action(ArgAction::SetTrue))
        // matches
        .get_matches();

    if matches.get_flag("version") {
        println!(
            "{} {} ({} {})",
            crate_name!(),
            crate_version!(),
            env!("LAST_COMMIT_ID"),
            env!("LAST_COMMIT_DATE"),
        );
        std::process::exit(0);
    }

    if matches.get_flag("license") {
        println!(include_str!("../../LICENSE.txt"));
        std::process::exit(0);
    }

    let runtime_options = vec![];

    aula_assistant_lib::run(runtime_options);
}
