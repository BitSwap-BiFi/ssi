// Self-sovereign identity
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2024 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2024 LNP/BP Standards Association. All rights reserved.
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

use amplify::Bytes32;
use baid58::{Chunking, FromBaid58, ToBaid58, CHUNKING_32};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[non_exhaustive]
pub enum Algo {
    #[default]
    Bip340,
    // Ed25519,
    Other(u8),
}

impl From<Algo> for u8 {
    fn from(algo: Algo) -> Self {
        match algo {
            Algo::Bip340 => 0,
            Algo::Other(v) => v,
        }
    }
}

impl From<u8> for Algo {
    fn from(value: u8) -> Self {
        match value {
            0 => Algo::Bip340,
            n => Algo::Other(n),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[non_exhaustive]
pub enum Chain {
    #[default]
    Bitcoin,
    Liquid,
    Other(u8),
}

impl From<Chain> for u8 {
    fn from(chain: Chain) -> Self {
        match chain {
            Chain::Bitcoin => 0,
            Chain::Liquid => 1,
            Chain::Other(v) => v,
        }
    }
}

impl From<u8> for Chain {
    fn from(value: u8) -> Self {
        match value {
            0 => Chain::Bitcoin,
            1 => Chain::Liquid,
            n => Chain::Other(n),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Ssi {
    pub algo: Algo,
    pub key: Bytes32,
    pub chain: Chain,
}

impl ToBaid58<34> for Ssi {
    const HRI: &'static str = "ssi";
    const CHUNKING: Option<Chunking> = CHUNKING_32;

    fn to_baid58_payload(&self) -> [u8; 34] { <[u8; 34]>::from(*self) }
}

impl From<Ssi> for [u8; 34] {
    fn from(ssi: Ssi) -> Self {
        let mut buf = [0u8; 34];
        buf[0] = ssi.algo.into();
        buf[33] = ssi.chain.into();
        buf[1..33].copy_from_slice(ssi.key.as_slice());
        buf
    }
}

impl From<[u8; 34]> for Ssi {
    fn from(value: [u8; 34]) -> Self {
        let algo = Algo::from(value[0]);
        let chain = Chain::from(value[33]);
        let key = Bytes32::from_slice_unsafe(&value[1..33]);
        Self { algo, key, chain }
    }
}

impl FromBaid58<34> for Ssi {}

impl Ssi {
    pub fn with_bip340(key: secp256k1::XOnlyPublicKey, chain: Chain) -> Self {
        Self {
            algo: Algo::Bip340,
            key: key.serialize().into(),
            chain,
        }
    }
}
