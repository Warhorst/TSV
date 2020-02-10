use crate::configuration::configuration_read_error::ConfigurationReadError;
use crate::configuration::configuration_reader::ConfigurationReader;
use crate::configuration::tsv_configuration::TSVConfiguration;
use std::fs::File;
use std::io::BufReader;

/// Reads the TSVConfiguration from a given JSON-File
pub struct JSONConfigurationReader {
    json_path: String
}

impl ConfigurationReader for JSONConfigurationReader {
    fn read_configuration(&self) -> Result<TSVConfiguration, ConfigurationReadError> {
        let file = File::open(&self.json_path)?;
        let reader = BufReader::new(file);

        Ok(serde_json::from_reader(reader)?)
    }
}