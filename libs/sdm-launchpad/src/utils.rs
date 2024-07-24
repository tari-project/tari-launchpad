// Copyright 2022. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

use anyhow::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use tauri::api::path::download_dir;
use walkdir::WalkDir;
use zip::{write::FileOptions, ZipWriter};

pub fn zip_and_export(in_path: String, zip_name: String, root_dir_name: Option<String>) -> Result<(), Error> {
    let zip_path = download_dir().unwrap().join(zip_name);
    let mut zip_writer = ZipWriter::new(File::create(zip_path)?);

    for entry in WalkDir::new(in_path.clone()) {
        let entry = entry?;
        let path = entry.path();
        let options = FileOptions::default();

        if path.is_file() {
            let file_path = match root_dir_name {
                Some(ref dir) => format!("{}/{}", dir, path.file_name().unwrap().to_string_lossy().into_owned()),
                None => path.strip_prefix(in_path.clone())?.to_string_lossy().into_owned(),
            };
            zip_writer.start_file(file_path, options)?;

            let mut file = File::open(path)?;
            let mut buffer: Vec<u8> = Vec::new();
            file.read_to_end(&mut buffer)?;
            zip_writer.write_all(&buffer)?;
        } else {
            let dir_path = path.strip_prefix(in_path.clone())?.to_str().unwrap();
            zip_writer.add_directory(dir_path, options)?;
        }
    }

    Ok(())
}
