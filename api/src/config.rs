use lazy_static::lazy_static;
use ory_kratos_client::apis::configuration::Configuration;

lazy_static! {
    pub static ref KRATOS_CONFIG: Configuration = {
        Configuration {
            base_path: "http://localhost:4433".to_string(),
            ..Default::default()
        }
    };
}
