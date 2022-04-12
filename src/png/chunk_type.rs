use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    /// Return this chunk type as a sequence of bytes described by the PNG spec.
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }

    fn _is_critical(&self) -> bool {
        self.0[0].is_ascii_uppercase()
    }

    fn _is_public(&self) -> bool {
        self.0[1].is_ascii_uppercase()
    }

    fn _is_reserved_bit_valid(&self) -> bool {
        self.0[2].is_ascii_uppercase()
    }

    fn _is_safe_to_copy(&self) -> bool {
        self.0[3].is_ascii_lowercase()
    }

    fn _is_valid(&self) -> bool {
        // The significance of the case of the third letter of the chunk name is reserved for possible future expansion.
        // At the present time all chunk names must have uppercase third letters.
        // (Decoders should not complain about a lowercase third letter, however, as some future version of the PNG
        // specification could define a meaning for this bit. It is sufficient to treat a chunk with a lowercase third
        // letter in the same way as any other unknown chunk type.)
        self._is_reserved_bit_valid()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(array: [u8; 4]) -> Result<Self, Self::Error> {
        if array.iter().all(|c| is_valid_type_code(*c)) {
            Ok(ChunkType(array))
        } else {
            Err(anyhow!("Type codes are restricted to consist of uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122 decimal)"))
        }
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(anyhow!("Type codes should be 4 bytes in length"));
        }

        if s.bytes().all(is_valid_type_code) {
            let mut arr = [0; 4];
            // Length of the str should be verified above, slice anyways so we don't panic
            arr.clone_from_slice(&s.as_bytes()[..4]);
            Ok(ChunkType(arr))
        } else {
            Err(anyhow!("Type codes are restricted to consist of uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122 decimal)"))
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.0.as_ref()))
    }
}

fn is_valid_type_code(ch: u8) -> bool {
    ch.is_ascii_lowercase() || ch.is_ascii_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk._is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk._is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk._is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk._is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk._is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk._is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk._is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk._is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk._is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk._is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
