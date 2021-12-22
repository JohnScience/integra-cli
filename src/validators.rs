fn check_if_has_manifest(presumable_crate_path: &str) -> Result<(), String> {
    let presumable_manifest_path = format!("{}{}Cargo.toml", presumable_crate_path, std::path::MAIN_SEPARATOR);
    match std::fs::metadata(&presumable_manifest_path) {
        Ok(_) => Ok(()),
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => Err(format!("Can't find {}", &presumable_manifest_path)),
                std::io::ErrorKind::PermissionDenied => Err(String::from("Permission denied")),
                // According to https://doc.rust-lang.org/std/fs/fn.metadata.html#errors
                // the match cases are exhaustive
                _ => unreachable!()
            }
        }
    }
}

fn check_if_lacks_integra_config(presumable_crate_path: &str) -> Result<(), String> {
    let presumable_integra_config_path = crate::ConfigTemplates::get_config_path_by_crate_path(presumable_crate_path);
    match std::fs::metadata(&presumable_integra_config_path) {
        // TODO: support overwriting the config file via a separate command
        Ok(_) => Err(format!("Integra config already exists at {}", &presumable_integra_config_path)),
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::PermissionDenied => Err(String::from("Permission denied")),
                std::io::ErrorKind::NotFound => Ok(()),
                // According to https://doc.rust-lang.org/std/fs/fn.metadata.html#errors
                // the match cases are exhaustive
                _ => unreachable!()
            }
        }
    }
}

pub fn contains_manifest_and_no_integra_config(presumable_crate_path: String) -> Result<(), String> {
    check_if_has_manifest(&presumable_crate_path)?;
    check_if_lacks_integra_config(&presumable_crate_path)
}