use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CpuTest {
    pub name: String,
    pub initial: InitialState,
    pub r#final: FinalState,
    pub cycles: Vec<Vec<Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitialState {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub ime: u8,
    pub ie: u8,
    pub ram: Vec<Vec<u16>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinalState {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub ime: u8,

    #[serde(default)]
    pub ei: u8,
    pub ram: Vec<Vec<u16>>,
}

