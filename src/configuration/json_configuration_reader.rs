use crate::configuration::configuration_reader::ConfigurationReader;
use crate::configuration::tsv_configuration::TSVConfiguration;

/// Reads the TSVConfiguration from a given JSON-File
pub struct JSONConfigurationReader {
    json_path: String
}

impl ConfigurationReader for JSONConfigurationReader {
    fn read_configuration(&self) -> TSVConfiguration {
        unimplemented!()
    }
}