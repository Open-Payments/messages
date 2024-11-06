// Suppress warnings about unused imports when features are not enabled
#![allow(unused_imports)]
use regex::Regex;

// Conditionally import necessary traits and modules
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};


#[derive(Debug)]
pub struct ValidationError {
    pub code: u32,
    pub message: String,
}

impl ValidationError {
    pub fn new(code: u32, message: String) -> Self {
        ValidationError { code, message }
    }
}
