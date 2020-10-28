use std::convert::TryFrom; 
use std::str::FromStr; 
use std::num::ParseIntError; 
use std::fmt; 

struct ChunkType {
    bytes: String
}

impl ChunkType {

    fn bytes(&self) -> [u8; 4]{
        return [0xB, 0xE, 0xE, 0xF]; 
    }

    fn is_valid(&self) -> bool{
        return true; 
    }

    fn is_critical(&self) -> bool{
        return true; 
    }

    fn is_public(&self) -> bool{
        return true; 
    }

    fn is_reserved_bit_valid(&self) -> bool {
        return true; 
    }

    fn is_safe_to_copy(&self) -> bool {
        return true; 
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str; 

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> { 
        if value[0] == 0xA {
            return Err("TODO: Do a valid byte check"); 
        }
        return Ok(ChunkType{
            bytes: "Ok".to_string()
        }); 
    }
}

impl FromStr for ChunkType {
    type Err = ParseIntError; 

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(ChunkType {
            bytes: s.to_string()
        }); 
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.bytes)
    }
}