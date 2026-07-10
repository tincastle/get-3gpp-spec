use directories::ProjectDirs;
use serde::Deserialize;
use std::path::PathBuf;

/// Application identifiers used to locate the platform config directory.
const QUALIFIER: &str = "engineer";
const ORGANIZATION: &str = "jeon";
const APPLICATION: &str = "get-3gpp-spec";

/// Settings loaded from `settings.toml`.
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub destination: String,
}

/// Return the path to `settings.toml` inside the platform's config directory.
///
/// Returns `None` if a config directory cannot be resolved for the current platform.
pub fn settings_path() -> Option<PathBuf> {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .map(|dirs| dirs.config_dir().join("settings.toml"))
}

/// Determine the download destination directory.
///
/// Resolves the settings file via [`settings_path`]; if it exists and parses as
/// valid TOML containing a `destination` field, that path is returned.
/// Otherwise (no file, unreadable, or malformed), falls back to the current
/// directory (`"."`).
pub fn resolve_destination() -> PathBuf {
    let path = match settings_path() {
        Some(p) => p,
        None => return PathBuf::from("."),
    };

    if !path.exists() {
        return PathBuf::from(".");
    }

    match std::fs::read_to_string(&path) {
        Ok(content) => match toml::from_str::<Settings>(&content) {
            Ok(settings) => PathBuf::from(settings.destination),
            Err(_) => PathBuf::from("."),
        },
        Err(_) => PathBuf::from("."),
    }
}

#[cfg(test)]
mod tests {
    use super::settings_path;

    #[test]
    fn settings_path_ends_with_settings_toml() {
        let path = settings_path().expect("config dir should resolve on this platform");
        assert_eq!(
            path.file_name().and_then(|n| n.to_str()),
            Some("settings.toml")
        );
    }
}
