use std::convert::TryFrom;
use std::fmt;
use std::io::{BufReader, Read};

use anyhow::{anyhow, Error, Result};

use crate::png::utils::crc_checksum;
use crate::png::ChunkType;

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    message_bytes: Vec<u8>,
    crc: u32,
}

impl Chunk {
    /// Create a new Chunk given the type and data
    ///
    /// * `chunk_type`: The type of the chunk
    /// * `message_bytes`: The chunk data
    pub fn new(chunk_type: ChunkType, message_bytes: Vec<u8>) -> Chunk {
        let crc = crc_checksum(&[chunk_type.bytes(), &message_bytes].concat());

        Self {
            length: message_bytes.len() as u32,
            chunk_type,
            message_bytes,
            crc,
        }
    }

    fn _length(&self) -> u32 {
        self.length
    }

    fn _chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn _data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.message_bytes.clone())?)
    }

    fn _crc(&self) -> u32 {
        self.crc
    }

    /// Return this chunk as a sequence of bytes described by the PNG spec.
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes())
            .chain(self.message_bytes.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(array: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(array);
        let mut arr = [0u8; 4];

        reader.read_exact(&mut arr)?;
        let length = u32::from_be_bytes(arr);

        reader.read_exact(&mut arr)?;
        let chunk_type = ChunkType::try_from(arr)?;

        let mut message_bytes = vec![0u8; usize::try_from(length)?];
        reader.read_exact(&mut message_bytes)?;

        reader.read_exact(&mut arr)?;
        let crc = u32::from_be_bytes(arr);

        // Validate crc checksum
        if crc == crc_checksum(&array[4..(length + 8) as usize]) {
            Ok(Self {
                length,
                chunk_type,
                message_bytes,
                crc,
            })
        } else {
            Err(anyhow!("Invalid CRC checksum"))
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::png::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk._length(), 42);
        assert_eq!(chunk._crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk._length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk._chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk._data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk._crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk._data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk._length(), 42);
        assert_eq!(chunk._chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk._crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
