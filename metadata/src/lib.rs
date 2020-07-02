// Copyright 2018-2019 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(test)]
mod tests;

pub mod layout2;
mod extension;
mod specs;
mod utils;

pub use self::extension::{
    InkProjectExtension,
    InkProjectSource,
    InkProjectContract,
    InkProjectUser,
};
pub use self::specs::{
    ConstructorSpec,
    ConstructorSpecBuilder,
    ContractSpec,
    ContractSpecBuilder,
    DisplayName,
    EventParamSpec,
    EventParamSpecBuilder,
    EventSpec,
    EventSpecBuilder,
    MessageParamSpec,
    MessageParamSpecBuilder,
    MessageSpec,
    MessageSpecBuilder,
    ReturnTypeSpec,
    TypeSpec,
};

#[cfg(feature = "derive")]
use scale_info::{
    form::CompactForm,
    IntoCompact as _,
    Registry,
};
use serde::Serialize;

const METADATA_VERSION: &str = "0.1.0";

/// An entire ink! project for ABI file generation purposes.
#[derive(Debug, Serialize)]
pub struct InkProject {
    metadata_version: semver::Version,
    #[serde(flatten)]
    extension: InkProjectExtension,
    spec: InkProjectSpec,
}

impl InkProject {
    pub fn new(extension: InkProjectExtension, spec: InkProjectSpec) -> Self {
        let metadata_version= semver::Version::parse(METADATA_VERSION)
            .expect("METADATA_VERSION is a valid semver string");
        InkProject {
            metadata_version,
            extension,
            spec
        }
    }
}

#[derive(Debug, Serialize)]
pub struct InkProjectSpec {
    #[serde(flatten)]
    registry: Registry,
    #[serde(rename = "storage")]
    layout: layout2::Layout<CompactForm>,
    spec: ContractSpec<CompactForm>,
}

impl InkProjectSpec {
    /// Creates a new ink! project.
    pub fn new<M, L, S>(layout: L, spec: S) -> Self
        where
            L: Into<layout2::Layout>,
            S: Into<ContractSpec>,
    {
        let mut registry = Registry::new();
        Self {
            layout: layout.into().into_compact(&mut registry),
            spec: spec.into().into_compact(&mut registry),
            registry,
        }
    }
}