use crate::configuration::tsv_configuration::TSVConfiguration;
use crate::configuration::configuration_read_error::ConfigurationReadError;

/// Reads a TSVConfiguration
pub trait ConfigurationReader {
    fn read_configuration(&self) -> Result<TSVConfiguration, ConfigurationReadError>;
}