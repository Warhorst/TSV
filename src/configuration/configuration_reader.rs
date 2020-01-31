use crate::configuration::tsv_configuration::TSVConfiguration;

/// Reads a TSVConfiguration
pub trait ConfigurationReader {
    fn read_configuration(&self) -> TSVConfiguration;
}