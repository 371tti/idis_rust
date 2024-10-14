// src/state_services/instruction_set.rs

use std::collections::HashMap;

pub struct  Instruction {
    pub api: String,
    pub target: Option<String>,
    pub body: Option<Vec<u8>>,
    pub parameters:  HashMap<String, String>,
    pub headers: HashMap<String, String>,
}