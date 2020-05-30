use std::time::SystemTime;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

const SIMPLEFLAKE_EPOCH: f64 = 946702800.0;
const SIMPLEFLAKE_TIMESTAMP_LENGTH: u64 = 41;
const SIMPLEFLAKE_RANDOM_LENGTH: u64 = 23;

const SIMPLEFLAKE_RANDOM_SHIFT: u64 = 0;
const SIMPLEFLAKE_TIMESTAMP_SHIFT: u64 = 23;


#[pyclass]
struct SimpleFlake {
    pub timestamp: f64,
    pub random_bits: u64,
}

#[pyfunction]
fn pad_bytes_to_64(data: usize) -> PyResult<String> {
    Ok(format!("{:064b}", data))
}

fn extract_bits(data: u64, shift: u64, length: u64) -> u64 {
    let bitmask = (1 << length - 1) << shift;
    (data & bitmask) >> shift
}

#[pyfunction]
fn simpleflake(timestamp: Option<f64>) -> PyResult<u64> {
    let given_time = timestamp.unwrap_or(SystemTime::now().
        duration_since(SystemTime::UNIX_EPOCH).expect("Time is not right").
        as_millis() as f64) as f64;
    let mil_sec = ((given_time - SIMPLEFLAKE_EPOCH) * 1000.0) as u64;
    let randomness = (rand::random::<u64>() % SIMPLEFLAKE_RANDOM_LENGTH) as u64;
    let result = ((mil_sec << SIMPLEFLAKE_TIMESTAMP_SHIFT) + randomness) as u64;
    Ok(result)
}

#[pyfunction]
fn parse_simpleflake(data: u64) -> PyResult<SimpleFlake> {
    let timestamp = SIMPLEFLAKE_EPOCH
        + extract_bits(data,
                       SIMPLEFLAKE_TIMESTAMP_SHIFT,
                       SIMPLEFLAKE_TIMESTAMP_LENGTH) as f64 / 1000.0;
    let random_bits = extract_bits(data,
                                   SIMPLEFLAKE_RANDOM_SHIFT,
                                   SIMPLEFLAKE_RANDOM_LENGTH);
    Ok(SimpleFlake {
        timestamp,
        random_bits,
    })
}


#[pymodule]
fn usimpleflake(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<SimpleFlake>()?;
    m.add_wrapped(wrap_pyfunction!(pad_bytes_to_64))?;
    m.add_wrapped(wrap_pyfunction!(parse_simpleflake))?;
    m.add_wrapped(wrap_pyfunction!(simpleflake))?;
    Ok(())
}
