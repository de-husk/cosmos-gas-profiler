use serde::{Deserialize, Serialize};
use std::{collections::HashMap, panic::Location};

use crate::client::chain_res::ChainResponse;

#[derive(PartialEq, Eq, Debug)]
pub enum CommandType {
    Store,
    Instantiate,
    Query,
    Execute,
    Migrate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GasProfiler {
    report: Report,
}

pub type Report = HashMap<String, HashMap<String, GasReport>>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GasReport {
    pub gas_wanted: u64,
    pub gas_used: u64,
    pub file_name: String,
    pub line_number: u32,
}

impl Default for GasProfiler {
    fn default() -> Self {
        Self::new()
    }
}

impl GasProfiler {
    pub fn new() -> Self {
        Self {
            report: HashMap::new(),
        }
    }

    pub fn instrument(
        &mut self,
        contract: String,
        op_name: String,
        op_type: CommandType,
        response: &ChainResponse,
        caller_loc: &CallLoc,
    ) {
        if op_type == CommandType::Query {
            // Wasm Query msgs don't cost gas
            return;
        }

        let op_key = format!("{:?}__{}", op_type, op_name);

        let m = self.report.entry(contract).or_default();
        m.insert(
            op_key,
            GasReport {
                gas_used: response.gas_used,
                gas_wanted: response.gas_wanted,
                file_name: caller_loc.file_name.clone(),
                line_number: caller_loc.line_number,
            },
        );
    }

    pub fn report(&self) -> &Report {
        &self.report
    }
}

pub struct CallLoc {
    pub file_name: String,
    pub line_number: u32,
}

impl From<&Location<'_>> for CallLoc {
    fn from(loc: &Location) -> CallLoc {
        CallLoc {
            file_name: loc.file().to_string(),
            line_number: loc.line(),
        }
    }
}
