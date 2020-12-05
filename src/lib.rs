//! A library for the Valve Data Model eXchange (DMX) format in Rust
use std::io::BufRead;

use anyhow::Result;

pub mod dmx;
pub mod formats;
mod read;
pub mod serde;

use crate::{
    dmx::File,
    read::{Readable, Slice},
};

/// Read a DMX file from an in-memory buffer, returns a borrowed version of the [dmx::File] struct
pub fn from_slice(reader: &[u8]) -> Result<File<&[u8], &str>> {
    File::read(&mut Slice(reader))
}

/// Read a DMX file from a (buffered) reader, returns an owned version of the [dmx::File] struct
pub fn from_reader(mut reader: impl BufRead) -> Result<File<Vec<u8>, String>> {
    File::read(&mut reader)
}
