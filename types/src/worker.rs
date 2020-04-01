// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::FunctionArguments;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::prelude::v1::*;

use anyhow;

pub trait TeaclaveRuntime {
    fn open_input(&self, identifier: &str) -> anyhow::Result<Box<dyn io::Read>>;
    fn create_output(&self, identifier: &str) -> anyhow::Result<Box<dyn io::Write>>;
}

pub trait TeaclaveFunction {
    fn execute(
        &self,
        runtime: Box<dyn TeaclaveRuntime + Send + Sync>,
        args: FunctionArguments,
    ) -> anyhow::Result<String>;
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum ExecutorType {
    Native,
    Python,
}

impl std::default::Default for ExecutorType {
    fn default() -> Self {
        ExecutorType::Native
    }
}

impl std::convert::TryFrom<&str> for ExecutorType {
    type Error = anyhow::Error;

    fn try_from(selector: &str) -> anyhow::Result<Self> {
        let sel = match selector {
            "python" => ExecutorType::Python,
            "native" | "platform" => ExecutorType::Native,
            _ => anyhow::bail!("Invalid executor selector: {}", selector),
        };
        Ok(sel)
    }
}

impl std::convert::From<ExecutorType> for String {
    fn from(executor_type: ExecutorType) -> String {
        format!("{}", executor_type)
    }
}

impl std::fmt::Display for ExecutorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ExecutorType::Native => write!(f, "native"),
            ExecutorType::Python => write!(f, "python"),
        }
    }
}

#[derive(Debug)]
pub struct WorkerCapability {
    pub runtimes: HashSet<String>,
    pub functions: HashSet<String>,
}

#[derive(Debug, Default)]
pub struct ExecutionResult {
    pub return_value: Vec<u8>,
    pub output_file_hash: HashMap<String, String>,
}

#[cfg(feature = "enclave_unit_test")]
pub mod tests {
    use super::*;
    //use crate::unit_tests;
    //use crate::unittest::*;

    pub fn run_tests() -> bool {
        //unit_tests!()
        true
    }
}
