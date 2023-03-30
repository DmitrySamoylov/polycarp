use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::BufReader,
    path::Path,
};

use anyhow::Context;
use fn_error_context::context;

#[context("Opening file '{path}' to write")]
pub fn open_to_write<P: AsRef<Path> + Display>(path: P) -> anyhow::Result<File> {
    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path.as_ref())
        .with_context(|| format!("Opening {path}"))
}

#[context("Opening file '{path}' to read")]
pub fn open_to_read<P: AsRef<Path> + Display>(path: P) -> anyhow::Result<BufReader<File>> {
    let file = File::open(path.as_ref()).with_context(|| format!("Opening {path}"))?;

    Ok(BufReader::new(file))
}
