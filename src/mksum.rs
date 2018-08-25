// rdiff(rust) -- library for network deltas
// Copyright 2018 Martin Pool.

//! Generate file signatures.
//!
//! Signatures describe a 'base' or 'old' file, and allow deltas to be generated without
//! access to the old file.

use std::io::{BufWriter, Read, Write, Result};
use byteorder::{BigEndian, WriteBytesExt};

use super::magic::SignatureFormat;

/// Configuration options for a generated signature file.
/// 
/// The values from `SignatureOptions::default()` are usually good, but applications
/// might want to set the `block_len`.
#[derive(Debug, Copy, Clone)]
pub struct SignatureOptions {
    /// Format of the signature, identified by its magic number.
    pub magic: SignatureFormat,

    /// Length of a block in bytes.
    /// 
    /// Smaller blocks produce larger signatures because there are more blocks, but allow matching
    /// smaller common regions between files.
    pub block_len: u32,

    /// Length of strong signatures.
    /// 
    /// This is normally best left at the default, which is the strong hash, but
    /// they may be truncated to get smaller signatures although with a risk of exploitable
    /// collisions.
    pub strong_len: u32,
}

impl SignatureOptions {
    pub fn default() -> SignatureOptions {
        SignatureOptions {
            magic: SignatureFormat::Blake2Sig,
            block_len: super::DEFAULT_BLOCK_LEN,
            strong_len: 8, // Whole Blake2 hash length.
        }
    }
}

fn write_u32be(f: &mut Write, a: u32) -> Result<()> {
    f.write_u32::<BigEndian>(a)
}

/// Generate a signature, reading a basis file and writing a signature file.
pub fn generate_signature(_basis: &mut Read, options: &SignatureOptions, sig: &mut Write) -> Result<()> {
    let mut sig = BufWriter::new(sig);
    write_u32be(&mut sig, options.magic as u32)?;
    write_u32be(&mut sig, options.block_len)?;
    write_u32be(&mut sig, options.strong_len)?;
    // TODO: Actually hash all the blocks!
    Ok(())
}

#[cfg(test)]
mod test {
    use std::vec::Vec;
    use std::io::Cursor;
    use super::*;
    
    #[test]
    pub fn empty_signature_header() {
        let mut sig_buf = Cursor::new(Vec::<u8>::new());
        let mut empty_input = Cursor::new(Vec::<u8>::new());
        let options = SignatureOptions::default();
        assert_eq!(options.block_len, 2 << 10);

        generate_signature(&mut empty_input, &options, &mut sig_buf).unwrap();
        assert_eq!(*sig_buf.get_ref(),
            [b'r', b's', 0x01, 0x37,  // BLAKE2 sig magic
            0, 0, 8, 0, // 2kB blocks
            0, 0, 0, 8, // 8 byte BLAKE2 hashes
            ]);
    }
}