// concat! macro can work only with string literals
macro_rules! get_template_lowercase_name {
    (GenericConfigTemplate) => { "generic" };
}

macro_rules! get_template_file_name_by_lowercase_name {
    ($template_lowercase_name:expr) => {
        concat!($template_lowercase_name, ".template.toml" )
    };
}

macro_rules! get_template_file_name_by_template_type {
    (GenericConfigTemplate) => {
        get_template_file_name_by_lowercase_name!(get_template_lowercase_name!(GenericConfigTemplate))
    };
}

// The macro relies on the relative position of the call site with respect to the config_templates directory
macro_rules! get_relative_path_to_template_file_by_template_type {
    (GenericConfigTemplate) => {
        // std::path::MAIN_SEPARATOR is not a literal but an expression
        // At the time of writing, host-specific conditional compilation is NOT required
        // because "/" separator works on all platforms if windows is not using long-path syntax
        concat!("config_templates", "/", get_template_file_name_by_template_type!(GenericConfigTemplate))
    };
}

pub enum ConfigTemplates {
    Generic(GenericConfigTemplate),
}

impl ConfigTemplates {
    pub fn get_config_path_by_crate_path(crate_path :&str) -> String {
        format!("{}{}Integra.toml", crate_path, std::path::MAIN_SEPARATOR)
    }

    pub const fn get_lowercase_name(&self) -> &'static str {
        match self {
            ConfigTemplates::Generic(_) => GenericConfigTemplate::get_lowercase_name(),
        }
    }

    pub fn try_write_config_template_on_fs(&self, crate_path: &str) -> Result<(), std::io::Error> {
        use std::io::Write;
        let mut file = std::fs::File::create(Self::get_config_path_by_crate_path(crate_path))?;
        match self {
            ConfigTemplates::Generic(_) => file.write_all(include_bytes!(get_relative_path_to_template_file_by_template_type!(GenericConfigTemplate))),
        }
    }

    // Internally calls try_write_config_template_on_fs, handles errors and reports the result
    pub fn generate_config_template(&self, crate_path: &str) {
        match self.try_write_config_template_on_fs(crate_path) {
            Ok(_) => println!(
                "{titlecase_attributive_adjective_phrase} Integra template generated at {config_path}",
                // https://en.wikipedia.org/wiki/Adjective_phrase#Attributive_vs._predicative
                titlecase_attributive_adjective_phrase = {
                    voca_rs::case::capitalize(self.get_lowercase_name(), /* rest_to_lower = */ false)
                },
                config_path = Self::get_config_path_by_crate_path(crate_path)
            ),
            Err(e) => match e.kind() {
                // Before generate_config_template() was called, the validator check_if_lacks_integra_config()
                //  ensured integra config wasn't available
                std::io::ErrorKind::AlreadyExists => unreachable!(),
                std::io::ErrorKind::PermissionDenied => eprintln!("Permission denied: {}", e.to_string()),
                _ => eprintln!("Unexpected I/O error: {}", e.to_string()),
            }
        }
    }

    pub fn generate_config_template_by_lowercase_name(name: &str, crate_path :&str) {
        // println!("generate_config_template_by_lowercase_name");
        match name {
            get_template_lowercase_name!(GenericConfigTemplate) => ConfigTemplates::Generic(GenericConfigTemplate {} ),
            // https://docs.rs/clap/latest/clap/struct.Arg.html#method.possible_values guarantees that
            //  only permissible values are accepted.
            _ => unreachable!(),
        }.generate_config_template(crate_path)
    }
}
pub struct GenericConfigTemplate;

impl GenericConfigTemplate {
    // TODO: generate the methods
    pub const fn get_lowercase_name() -> &'static str { get_template_lowercase_name!(GenericConfigTemplate) }
    pub const fn get_template_file_name() -> &'static str { get_template_file_name_by_template_type!(GenericConfigTemplate) }
    pub const fn as_enum_variant() -> ConfigTemplates { ConfigTemplates::Generic(GenericConfigTemplate {}) }
    pub const fn get_template_file_contents() -> &'static str {
        // std::path::MAIN_SEPARATOR is not a literal but an expression
        // At the time of writing, host-specific conditional compilation is NOT required
        // because "/" separator works on all platforms if windows is not using long-path syntax
        include_str!(concat!("config_templates", "/", get_template_file_name_by_template_type!(GenericConfigTemplate)))
    }
}