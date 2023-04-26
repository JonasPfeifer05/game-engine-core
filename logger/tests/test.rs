use std::fs;
use log::{debug, error, info, LevelFilter, trace, warn};

const PATH_TO_TEST_FILE: &str = "./resources/_test.txt";

fn get_test_result() -> String {
    let data = fs::read_to_string(PATH_TO_TEST_FILE).expect("Failed to read from file!");
    fs::remove_file(PATH_TO_TEST_FILE).expect("Failed to remove file!");
    data
}

#[test]
fn test_max_level_error() {
    let expected_result = "[ERROR]: ERROR\n";
    logger::templates::init_simple_file_logger(LevelFilter::Error, PATH_TO_TEST_FILE).expect("Failed to init logger!");

    error!("ERROR");
    warn!("WARN");
    info!("INFO");
    debug!("DEBUG");
    trace!("TRACE");

    assert_eq!(
        expected_result,
        get_test_result()
    );
}

#[test]
fn test_max_level_warn() {
    let expected_result = "[ERROR]: ERROR\n [WARN]: WARN\n";
    logger::templates::init_simple_file_logger(LevelFilter::Warn, PATH_TO_TEST_FILE).expect("Failed to init logger!");

    error!("ERROR");
    warn!("WARN");
    info!("INFO");
    debug!("DEBUG");
    trace!("TRACE");

    assert_eq!(
        expected_result,
        get_test_result()
    );
}

#[test]
fn test_max_level_info() {
    let expected_result = "[ERROR]: ERROR\n [WARN]: WARN\n [INFO]: INFO\n";
    logger::templates::init_simple_file_logger(LevelFilter::Info, PATH_TO_TEST_FILE).expect("Failed to init logger!");

    error!("ERROR");
    warn!("WARN");
    info!("INFO");
    debug!("DEBUG");
    trace!("TRACE");

    assert_eq!(
        expected_result,
        get_test_result()
    );
}

#[test]
fn test_max_level_debug() {
    let expected_result = "[ERROR]: ERROR\n [WARN]: WARN\n [INFO]: INFO\n[DEBUG]: DEBUG\n";
    logger::templates::init_simple_file_logger(LevelFilter::Debug, PATH_TO_TEST_FILE).expect("Failed to init logger!");

    error!("ERROR");
    warn!("WARN");
    info!("INFO");
    debug!("DEBUG");
    trace!("TRACE");

    assert_eq!(
        expected_result,
        get_test_result()
    );
}

#[test]
fn test_max_level_trace() {
    let expected_result = "[ERROR]: ERROR\n [WARN]: WARN\n [INFO]: INFO\n[DEBUG]: DEBUG\n[TRACE]: TRACE\n";
    logger::templates::init_simple_file_logger(LevelFilter::Trace, PATH_TO_TEST_FILE).expect("Failed to init logger!");

    error!("ERROR");
    warn!("WARN");
    info!("INFO");
    debug!("DEBUG");
    trace!("TRACE");

    assert_eq!(
        expected_result,
        get_test_result()
    );
}