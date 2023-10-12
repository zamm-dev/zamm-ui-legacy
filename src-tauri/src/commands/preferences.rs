use anyhow::anyhow;
use path_absolutize::Absolutize;
use serde::{Deserialize, Serialize};

use specta::{specta, Type};
use std::fs;

use std::path::PathBuf;

use crate::commands::errors::ZammResult;

static PREFERENCES_FILENAME: &str = "preferences.yaml";

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub struct Preferences {
    unceasing_animations: Option<bool>,
    sound_on: Option<bool>,
}

fn get_preferences_happy_path(
    maybe_preferences_dir: Option<&PathBuf>,
) -> ZammResult<Preferences> {
    let preferences_dir =
        maybe_preferences_dir.ok_or(anyhow!("No preferences dir found"))?;
    let relative_preferences_path = preferences_dir.join(PREFERENCES_FILENAME);
    let preferences_path = relative_preferences_path.absolutize()?;
    let display_filename = preferences_path.display();
    if preferences_path.exists() {
        println!("Reading preferences from {display_filename}");
        let contents = fs::read_to_string(preferences_path)?;
        let preferences: Preferences = serde_yaml::from_str(&contents)?;
        Ok(preferences)
    } else {
        println!("No preferences found at {display_filename}");
        Ok(Preferences::default())
    }
}

fn get_preferences_helper(preferences_path: &Option<PathBuf>) -> Preferences {
    match get_preferences_happy_path(preferences_path.as_ref()) {
        Ok(preferences) => preferences,
        Err(e) => {
            eprintln!("Error getting preferences: {e}");
            Preferences::default()
        }
    }
}

#[tauri::command]
#[specta]
pub fn get_preferences(app_handle: tauri::AppHandle) -> Preferences {
    let app_dir = app_handle.path_resolver().app_config_dir();
    get_preferences_helper(&app_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_call::SampleCall;

    use std::fs;

    fn read_sample(filename: &str) -> SampleCall {
        let sample_str = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("No file found at {filename}"));
        serde_yaml::from_str(&sample_str).unwrap()
    }

    fn check_get_preferences_sample(file_prefix: &str, preferences_dir: &str) {
        let sample = read_sample(file_prefix);
        assert_eq!(sample.request, vec!["get_preferences"]);

        let actual_result = get_preferences_helper(&Some(preferences_dir.into()));
        let actual_json = serde_json::to_string_pretty(&actual_result).unwrap();
        let expected_json = sample.response.trim();
        assert_eq!(actual_json, expected_json);
    }

    #[test]
    fn test_get_preferences_without_file() {
        check_get_preferences_sample(
            "./api/sample-calls/get_preferences-no-file.yaml",
            "./non-existent/path",
        );
    }

    #[test]
    fn test_get_preferences_happy_path_without_file() {
        let non_existent_path = PathBuf::from("./non-existent/path");
        let happy_path_result = get_preferences_happy_path(Some(&non_existent_path));
        assert!(happy_path_result.is_ok());
        assert_eq!(happy_path_result.unwrap(), Preferences::default());
    }

    #[test]
    fn test_get_preferences_with_sound_override() {
        check_get_preferences_sample(
            "./api/sample-calls/get_preferences-sound-override.yaml",
            "./api/sample-settings/sound-override",
        );
    }

    #[test]
    fn test_get_preferences_with_extra_settings() {
        check_get_preferences_sample(
            "./api/sample-calls/get_preferences-extra-settings.yaml",
            "./api/sample-settings/extra-settings",
        );
    }
}
