#![feature(const_fmt_arguments_new)]

use core::time;
use std::{mem::MaybeUninit, path::PathBuf};

use clap::{Arg, App, SubCommand, crate_authors, crate_version};
use integra_cli::ConfigTemplates;

mod lib;
mod validators;
mod handlers;

// TODO: create macros for clap that would verify correctness and lessen the boilerplate
mod subcommands {
    pub use crate::handlers::handle;
    pub mod config {
        pub const SC_NAME: &'static str = "cfg";
        pub use crate::handlers::config::handle;
        pub mod init {
            pub const SC_NAME: &'static str = "init";
            pub use crate::handlers::config::init::handle;
            pub mod args {
                pub mod template {
                    pub const ARG_ENAME: &'static str = "TEMPLATE";
                    pub const ARG_LNAME: &'static str = "template";
                    pub const ARG_SNAME: &'static str = "t";
                    pub mod possible_values {
                        use integra_cli::GenericConfigTemplate;

                        pub const GENERIC: &'static str = GenericConfigTemplate::get_lowercase_name();
                    }
                    pub const POSSIBLE_VALUES: &'static [&'static str; 1] = &[self::possible_values::GENERIC];
                    pub const DEFAULT_VALUE: &'static str = possible_values::GENERIC;
                }
                pub mod crate_path {
                    pub const ARG_ENAME: &'static str = "CRATE_PATH";
                    pub const ARG_LNAME: &'static str = "crate_path";
                    pub const ARG_SNAME: &'static str = "p";
                    pub use crate::validators::contains_manifest_and_no_integra_config;
                }
            }
        }
    }
}

fn main() {
    let mut current_dir: MaybeUninit<PathBuf> = MaybeUninit::uninit();
    let cli = App::new("Integra")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Integration tool for Rust")
        .subcommand(SubCommand::with_name(subcommands::config::SC_NAME)
                .about("set of subcommands for working with local Integra config files")
                .subcommand(SubCommand::with_name(subcommands::config::init::SC_NAME)
                    .about("generates a generic or use-specific Integra config file template
                    in the current directory or some specified path")
                    .arg(Arg::with_name(subcommands::config::init::args::template::ARG_ENAME)
                        .long(subcommands::config::init::args::template::ARG_LNAME)
                        .short(subcommands::config::init::args::template::ARG_SNAME)
                        .possible_values(subcommands::config::init::args::template::POSSIBLE_VALUES)
                        .default_value(subcommands::config::init::args::template::DEFAULT_VALUE)
                        .takes_value(true)
                    )
                    .arg(Arg::with_name(subcommands::config::init::args::crate_path::ARG_ENAME)
                        .long(subcommands::config::init::args::crate_path::ARG_LNAME)
                        .short(subcommands::config::init::args::crate_path::ARG_SNAME)
                        .default_value({
                            // At the time of writing, Arg doesn't offer a way to initialize the
                            //  default argument lazily
                            // https://docs.rs/clap/latest/clap/struct.Arg.html
                            current_dir.write(std::env::current_dir().unwrap()).to_str().unwrap()
                        })
                        .hide_default_value(true)
                        .validator(crate::validators::contains_manifest_and_no_integra_config)
                        .takes_value(true)
                    )
                )
            )
        .get_matches();

    subcommands::handle(&cli);
}