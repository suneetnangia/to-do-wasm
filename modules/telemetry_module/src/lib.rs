// wit_bindgen_rust::import!("../../wits/hostfunctions.wit");
wit_bindgen_rust::export!("../../wits/wasmtelemetryfunctions.wit");

mod iot_config;
use std::fs;
// use anyhow::{Result};

const MODULE_NAME: &str = "Simulated Temperature Sensor Edge Module";

struct Wasmtelemetryfunctions;

impl wasmtelemetryfunctions::Wasmtelemetryfunctions for Wasmtelemetryfunctions {
   
    // Initialise module with required configuration.
    fn init(config_file_path: String)
    {
        let configfilepath = config_file_path;
        let configfilecontents = fs::read_to_string(configfilepath).unwrap(); // TODO: use anyhow::Result instead of unwrap.

        let iot_edge_config = iot_config::Configuration::new(configfilecontents);
        let iot_edge_connection_string = iot_edge_config.connection_string();
        
        println!("Initialising edge module '{}' with connection string '{}'", MODULE_NAME, iot_edge_connection_string);

        // Calling host function here and print the return value.
        hostfunctions::sendtelemetry(r#"{"device Id" : "001", "temp" : 12.5, "pressure": 20.3}"#);
    }
}

// #[cfg(test)]
// mod module_tests {
//     use super::*;
//     #[test]
//     fn check_module_init() {
//         Wasmtelemetryfunctions::init("config.toml".to_string());
//     }
// }