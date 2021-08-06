use std::{io, fs};

use ubiquity::configuration;
use mockito;

pub fn config_from_url(url: String) -> configuration::Configuration {
  return configuration::Configuration {
    base_path: url,
    ..configuration::Configuration::new()
  };
}

pub fn create_mock_from_file(
  filepath: &str,
  request_path: &str,
) -> Result<mockito::Mock, io::Error> {
  let mock_content = fs::read_to_string(filepath)?;
  return Ok(
    mockito::mock("GET", request_path)
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_content)
      .create(),
  );
}
