use std::{io, fs};

use mockito;

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
