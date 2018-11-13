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

use std::io::{Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::{env, fs, io};

pub fn add<P: AsRef<Path>>(fileset: &str, dest_path: P) -> io::Result<()> {
    enter_fileset_dir(fileset)?;

    fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(base64::encode(dest_path.as_ref().as_os_str().as_bytes()))
        .and_then(|mut f| {
            let mut buffer = Vec::new();
            let stdin = io::stdin();
            stdin.lock().read_to_end(&mut buffer)?;
            f.write_all(&buffer)
        })
}

pub fn rm<P: AsRef<Path>>(fileset: &str, dest_path: &Option<P>) -> io::Result<()> {
    match dest_path {
        Some(dest_path) => {
            enter_fileset_dir(fileset)?;
            fs::remove_file(base64::encode(dest_path.as_ref().as_os_str().as_bytes()))
        }
        None => fs::remove_dir_all(base64::encode(fileset)),
    }
}

pub fn ls(fileset: &Option<String>) -> io::Result<()> {
    if let Some(fileset) = fileset {
        enter_fileset_dir(fileset)?;
    }
    for e in fs::read_dir(".")? {
        let path64 = e?.path();
        let stdout = io::stdout();
        let mut stdout_handle = stdout.lock();
        base64::decode(&path64.as_os_str().as_bytes()[2..])
            .map(|path| {
                stdout_handle.write_all(&path)?;
                stdout_handle.write_all(b"\n")
            }).map_err(|_| eprintln!("Invalid file name"))
            .ok();
    }
    Ok(())
}

pub fn get<P: AsRef<Path>>(fileset: &str, dest_path: P) -> io::Result<()> {
    enter_fileset_dir(fileset)?;

    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();
    stdout_handle.write_all(fs::read(base64::encode(
        dest_path.as_ref().as_os_str().as_bytes(),
    ))?.as_ref())
}

fn enter_fileset_dir(fileset: &str) -> io::Result<()> {
    let dir_name = base64::encode(fileset);
    let dir_path = Path::new(&dir_name);
    fs::DirBuilder::new().create(dir_path).ok();
    env::set_current_dir(dir_path)
}
