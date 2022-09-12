use lazy_static::lazy_static;
use ory_kratos_client::apis::configuration::Configuration;

lazy_static! {
    #[derive(Debug)]
    pub static ref KRATOS_CONFIG: Configuration = {
        Configuration {
            base_path: std::env::var("KRATOS_URL").unwrap_or_else(|_| "http://localhost:4434".to_owned()),
            ..Default::default()
        }
    };
}
