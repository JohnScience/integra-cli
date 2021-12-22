// All calls of handlers must happen from crate::subcommands::

#[macro_export]
macro_rules! explain { 
    ($s:expr) => { println!("{}. Call with --help flag for more information", $s) };
}

#[macro_export]
macro_rules! requires_arguments {
    ($subcommand_name:expr) => {
        crate::explain!(format!("{} subcommand requires arguments", $subcommand_name))
    };
}

pub fn handle(cli: &clap::ArgMatches) {
    match cli.subcommand() {
        (crate::subcommands::config::SC_NAME, opt_config_args) => { 
            crate::subcommands::config::handle(opt_config_args)
        },
        ("", None) => explain!("No subcommand was used"),
        _ => explain!("Invalid subcommand"),
    }
}

pub mod config {
    pub fn handle(opt_matches: Option<&clap::ArgMatches>) {
        match opt_matches {
            None => crate::requires_arguments!(crate::subcommands::config::SC_NAME),
            Some(config_args) => {
                match config_args.subcommand() {
                    (
                        crate::subcommands::config::init::SC_NAME,
                        opt_config_init_args
                    ) => crate::subcommands::config::init::handle(opt_config_init_args),
                    _ => crate::explain!("Invalid subcommand"),
                }
            }
        }
    }

    pub mod init {
        use integra_cli::ConfigTemplates;

        pub fn handle(opt_matches: Option<&clap::ArgMatches>) {
            let matches = opt_matches.unwrap();
            
            let crate_path = matches.value_of(crate::subcommands::config::init::args::crate_path::ARG_ENAME)
                // https://docs.rs/clap/latest/clap/struct.Arg.html#method.default_value guarantees that
                //  there is always an argument    
                .unwrap();
            match matches.value_of(crate::subcommands::config::init::args::template::ARG_ENAME) {
                // https://docs.rs/clap/latest/clap/struct.Arg.html#method.default_value guarantees that
                //  there is always an argument
                None => unreachable!(),
                Some(name) => ConfigTemplates::generate_config_template_by_lowercase_name(name, crate_path),
            }
        }
    }
}