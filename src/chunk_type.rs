use std::str::FromStr;
use core::fmt::{Display, Formatter};

#[derive(Debug)]
struct ChunkType {
    bytes: u32,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes.to_be_bytes()
    }

    pub fn is_valid(&self) -> bool {
        todo!()
    }

    pub fn is_critical(&self) -> bool {
        todo!()
    }

    pub fn is_public(&self) -> bool {
        todo!()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        todo!()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        todo!()
    }

}

#[derive(Debug)]
struct ChunkConversionError {
    message: String,
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkConversionError;

    fn try_from(byte_stream: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType {
            bytes: u32::from_be_bytes(byte_stream),
        })
    }
}

impl FromStr for ChunkType {
    type Err = ChunkConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            Err ( ChunkConversionError { message: "String of bytes was not 4 bytes long, so cannot be converted to PDF ChunkType".to_owned() } )
        } else {
            Ok(ChunkType {
                bytes: s.bytes().fold(0, |acc, x| acc + x as u32),
            })
        }

    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            let bytes = self.bytes();
            write!(f, "[{}, {}, {}, {}]", bytes[0], bytes[1], bytes[2], bytes[3])
        }
}

impl PartialEq<ChunkType> for ChunkType {
    fn eq(&self, other: &ChunkType) -> bool {
        self.bytes == other.bytes
    }
}

impl Eq for ChunkType {}


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
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

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
