use std::error;
use std::fmt;
use anyhow::{Context, Result};
use serde::Deserialize;
use thiserror::Error;

#[derive(PartialEq, Debug)]
pub enum CreationError {
    Negative,
    Zero,
}

// manual error method
impl error::Error for CreationError {}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CreationError::Negative => write!(f, "CreationError: Negative"),
            CreationError::Zero => write!(f, "CreationError: Zero"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    pub fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}


// thiserror method
#[derive(Error, Debug)]
pub enum MyError {
    #[error("A gizmo error occurred!")]
    GizmoError,
    #[error("Could not find widget '{name}'")]
    WidgetNotFoundError { name: String },
}

#[derive(PartialEq, Debug)]
pub struct NegativeNonzeroInteger(i64);

impl NegativeNonzeroInteger {
    pub fn new(value: i64) -> Result<NegativeNonzeroInteger, MyError> {
        match value {
            x if x > 0 => Err(MyError::GizmoError),
            x if x == 0 => Err(MyError::WidgetNotFoundError{name: "zero".to_string()}),
            x => Ok(NegativeNonzeroInteger(x as i64)),
        }
    }
}

// anyhow method
#[derive(Debug, Deserialize)]
pub struct ClusterMap { 
    x: u32,
    y: u32
}

pub fn get_cluster_info() -> Result<ClusterMap> {
    let config = std::fs::read_to_string("D:\\s\\Fe2O3\\learn\\5-error-handling\\src\\cluster1.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}

pub fn get_cluster_info_with_context() -> Result<()> {
    let path = "D:\\s\\Fe2O3\\learn\\5-error-handling\\src\\cluster1.json";
    let _content = std::fs::read(path)
        .with_context(|| format!("Failed to read instrs from {}", path))?;
    
    Ok(())
}

fn string_error() -> Result<()> {
    Err(anyhow::anyhow!("invalid format. value is empty"))
}

pub fn io_error() -> Result<()> {
    Err(anyhow::anyhow!(std::io::Error::new(std::io::ErrorKind::Other, "invalid IO packet")))
}

pub fn any_error() -> Result<()> {
    string_error()?;
    io_error()?;
    Ok(())
}
