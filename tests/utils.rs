use std::{fs, io};

use ubiquity::configuration;

const VERSION: &str = "/v1";

pub fn config_from_url(url: String) -> configuration::Configuration {
    configuration::Configuration {
        base_path: get_base_path(url, VERSION),
        ..configuration::Configuration::new()
    }
}

pub fn get_string_from_file(filepath: &str) -> String {
    return fs::read_to_string(filepath).expect("error in read file to string");
}

pub fn create_mock_from_file(
    filepath: &str,
    request_path: &str,
) -> Result<mockito::Mock, io::Error> {
    let mock_content = get_string_from_file(filepath);

    return create_mock(&mock_content, request_path);
}

pub fn create_mock(body: &str, request_path: &str) -> Result<mockito::Mock, io::Error> {
    let path: &str = &get_path(VERSION, request_path);

    Ok(mockito::mock("GET", path)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(body)
        .create())
}

fn get_path(version: &str, request_path: &str) -> String {
    return format!("{}{}", version, request_path);
}

fn get_base_path(url: String, version: &str) -> String {
    return format!("{}{}", url, version);
}

pub struct Setup {
    pub mocks: Vec<mockito::Mock>,
    pub config: configuration::Configuration,
}

pub fn new_setup(url: String, mocks: Vec<mockito::Mock>) -> Setup {
    return Setup {
        mocks,
        config: config_from_url(url),
    };
}
