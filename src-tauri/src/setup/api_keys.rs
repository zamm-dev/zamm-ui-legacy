use serde::{Deserialize, Serialize};
use specta::Type;
use std::env;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub enum Source {
    Environment,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub struct ApiKey {
    pub value: String,
    pub source: Source,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub struct ApiKeys {
    pub openai: Option<ApiKey>,
}

pub fn setup_api_keys() -> ApiKeys {
    let mut api_keys = ApiKeys { openai: None };
    if let Ok(openai_api_key) = env::var("OPENAI_API_KEY") {
        api_keys.openai = Some(ApiKey {
            value: openai_api_key,
            source: Source::Environment,
        });
    }
    api_keys
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_env;

    #[test]
    fn test_get_empty_api_keys() {
        temp_env::with_var("OPENAI_API_KEY", None::<String>, || {
            let api_keys = setup_api_keys();
            assert!(api_keys.openai.is_none());
        });
    }

    #[test]
    fn test_get_present_api_keys() {
        temp_env::with_var("OPENAI_API_KEY", Some("dummy"), || {
            let api_keys = setup_api_keys();
            assert_eq!(
                api_keys.openai,
                Some(ApiKey {
                    value: "dummy".to_string(),
                    source: Source::Environment,
                })
            );
        });
    }
}
