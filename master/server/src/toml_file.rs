use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::fs;
use std::path::Path;
use toml;

/// Trait shares read from file and write to file behavior between toml-serialized structures
pub trait ReadAndWriteTOMLFile<T: DeserializeOwned> {
    fn read_from_file(path: &Path) -> Result<T, Box<dyn Error>> {
        let toml_file_content = fs::read_to_string(path)?;
        let result = toml::from_str(&toml_file_content)?;
        Ok(result)
    }

    fn write_to_file(self: &Self, path: &Path) -> Result<(), Box<dyn Error>>
    where
        Self: Serialize,
    {
        let toml_file_content = toml::to_string(&self)?;
        fs::write(path, toml_file_content)?;
        Ok(())
    }
}
