// Copyright (c) 2018, [Ribose Inc](https://www.ribose.com).
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

extern crate nereon;

use self::nereon::{parse_noc, ConversionError, FromValue, Value};
use std::env::args_os;
use std::path::PathBuf;

const NOS: &str = include_str!("nos");
const LICENSE: &str = "BSD-2-Clause";

#[derive(FromValue)]
pub struct Config {
    pub fileset_dir: PathBuf,
    pub command: Command,
}

#[derive(FromValue)]
pub enum Command {
    Add { fileset: String, path: PathBuf },
    Rm { fileset: String, path: Option<PathBuf> },
    Ls { fileset: Option<String> },
    Get { fileset: String, path: PathBuf },
}

pub fn configure() -> Config {
    let noc = parse_noc::<Value>(NOS)
        .unwrap_or_else(|e| panic!("Invalid NOC: {:?}", e))
        .insert(vec!["version"], Value::from(env!("CARGO_PKG_VERSION")))
        .insert(
            vec!["authors"],
            Value::from(env!("CARGO_PKG_AUTHORS").split(":").collect::<Vec<_>>()),
        ).insert(vec!["license"], Value::from(LICENSE))
        .insert(vec!["name"], Value::from(env!("CARGO_PKG_NAME")))
        .insert(vec!["about"], Value::from(env!("CARGO_PKG_DESCRIPTION")));

    nereon::configure(noc, args_os()).unwrap()
}
