use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Version(String);

// TODO: Similarly to the way how crates are specified as dependencies, support more options
//  for specifying crates producing artifacts
#[derive(Serialize, Deserialize)]
pub enum Crate {
    CrateWithKnownVersion(Version),
}

// Unfortunately, while TOML itself supports comments, it is not the case for toml-rs aka toml crate
// https://github.com/alexcrichton/toml-rs/issues/274
#[derive(Serialize, Deserialize, Default)]
pub struct GenericConfig {
    // Rust crate > world
    export_artifacts: std::collections::HashMap<String, Crate>,
    // world < Rust crate
    import_artifacts: std::collections::HashMap<String, Crate>,
    // Rust crate <- sync -> world
    #[serde(rename = "sync")]
    sync_utils: std::collections::HashMap<String, Crate>,
}