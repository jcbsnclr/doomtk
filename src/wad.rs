use std::fs;
use std::io::{self, prelude::*};
use std::path::Path;

#[derive(thiserror::Error, Debug)]
pub enum WadError {
    #[error(
        "invalid WAD kind: header should start with 'IWAD' or 'PWAD', found {0:?} ('{}')",
        .0.escape_ascii()
    )]
    InvalidKind([u8; 4]),
    #[error("missing header")]
    MissingHeader,

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

type Result<T> = std::result::Result<T, WadError>;

pub enum Kind {
    Iwad,
    Pwad,
}

pub struct Wad {
    kind: Kind,
    dir_len: usize,
    dir_ptr: usize,

    data: Box<[u8]>,
}

impl Wad {
    const HEADER_LEN: usize = 12;

    /// Opens a WAD file at a given path.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Wad> {
        // read file into memory
        let data = fs::read(path)?.into_boxed_slice();

        // extract header
        let header = data
            .get(..Self::HEADER_LEN)
            .ok_or(WadError::MissingHeader)?;

        // create cursor for reading fields of header
        let mut cursor = io::Cursor::new(header);

        // parse kind of wad
        let kind = match &cursor.read_array()? {
            b"IWAD" => Kind::Iwad,
            b"PWAD" => Kind::Pwad,

            k => return Err(WadError::InvalidKind(*k)),
        };

        // read directory length and pointer
        let dir_len = cursor.read_int()? as usize;
        let dir_ptr = cursor.read_int()? as usize;

        Ok(Wad {
            kind,
            dir_len,
            dir_ptr,
            data,
        })
    }
}

/// Helper methods for working with a [`Read`]er
trait ReadExt: Read {
    /// Reads exactly `N` bytes and returns an array
    fn read_array<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        let mut buf = [0; N];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Reads a single 32-bit integer
    fn read_int(&mut self) -> io::Result<i32> {
        let array = self.read_array()?;
        Ok(i32::from_le_bytes(array))
    }
}

impl<T> ReadExt for T where T: Read {}
