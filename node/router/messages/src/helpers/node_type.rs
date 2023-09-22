// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkOS library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[repr(u8)]
pub enum NodeType {
    /// A client node is a full node, capable of syncing with the network.
    Client = 0,
    /// A prover is a light node, capable of producing proofs for consensus.
    Prover,
    /// A validator is a full node, capable of validating blocks.
    Validator,
}

impl NodeType {
    /// Returns a string representation of the node type.
    pub const fn description(&self) -> &str {
        match self {
            Self::Client => "a client node",
            Self::Prover => "a prover node",
            Self::Validator => "a validator node",
        }
    }

    /// Returns `true` if the node type is a client.
    pub const fn is_client(&self) -> bool {
        matches!(self, Self::Client)
    }

    /// Returns `true` if the node type is a prover.
    pub const fn is_prover(&self) -> bool {
        matches!(self, Self::Prover)
    }

    /// Returns `true` if the node type is a validator.
    pub const fn is_validator(&self) -> bool {
        matches!(self, Self::Validator)
    }
}

impl core::fmt::Display for NodeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", match self {
            Self::Client => "Client",
            Self::Prover => "Prover",
            Self::Validator => "Validator",
        })
    }
}
