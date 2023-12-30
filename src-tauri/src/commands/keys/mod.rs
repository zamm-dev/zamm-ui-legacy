mod get;
mod set;

pub use get::get_api_keys;
pub use set::set_api_key;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup::api_keys::ApiKeys;
    use crate::ZammApiKeys;
    use get::tests::check_get_api_keys_sample;
    use set::tests::check_set_api_key_sample;
    use std::sync::Mutex;

    #[test]
    fn test_get_after_set() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));

        check_set_api_key_sample(
            "api/sample-calls/set_api_key-existing-no-newline.yaml",
            &api_keys,
            "api_keys_integration_tests",
        );

        check_get_api_keys_sample(
            "./api/sample-calls/get_api_keys-openai.yaml",
            &api_keys,
        );
    }
}
