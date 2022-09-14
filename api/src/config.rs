use lazy_static::lazy_static;
use supertokens::apis::configuration::Configuration;

lazy_static! {
    #[derive(Debug)]
    pub static ref SUPERTOKENS_CONFIG: Configuration = {
        Configuration {
            base_path: std::env::var("SUPERTOKENS_URL").unwrap_or_else(|_| "http://localhost:4434".to_owned()),
            ..Default::default()
        }
    };
}
