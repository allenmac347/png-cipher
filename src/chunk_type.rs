use std::str;
use std::str::FromStr; 
use std::convert::TryFrom; 
use std::fmt; 

#[derive(Debug)]
struct ChunkType {
    bytes: String
}

impl ChunkType {

    fn bytes(&self) -> [u8; 4]{
        let mut bytes_array: [u8; 4] = [0, 0, 0, 0]; 
        for (i,b) in self.bytes.as_bytes().iter().enumerate(){
            bytes_array[i] = *b; 
        }
        return bytes_array; 
    }

    fn is_valid(&self) -> bool{
        for (i,byte) in self.bytes.as_bytes().iter().enumerate(){
            if i != 2 && !byte.is_ascii_lowercase() && !byte.is_ascii_uppercase(){
                return false; 
            }
            if i == 2 && !byte.is_ascii_uppercase(){
                return false; 
            }
        }
        return true; 
    }

    fn is_critical(&self) -> bool{
        let byte_one = self.bytes.as_bytes()[0]; 
        if ((byte_one >> 5) & 1) == 1{
            return false; 
        }
        return true; 
    }

    fn is_public(&self) -> bool{
        let byte_one = self.bytes.as_bytes()[1]; 
        if ((byte_one >> 5) & 1) == 1{
            return false; 
        }
        return true;  
    }

    fn is_reserved_bit_valid(&self) -> bool {
        let byte_one = self.bytes.as_bytes()[2]; 
        if ((byte_one >> 5) & 1) == 1{
            return false; 
        }
        return true;  
    }

    fn is_safe_to_copy(&self) -> bool {
        let byte_one = self.bytes.as_bytes()[3]; 
        if ((byte_one >> 5) & 1) == 0{
            return false; 
        }
        return true;  
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str; 

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> { 
        for b in value.iter(){
            if (b < &65) || (&90 < b && b < &97) || (&122 < b){
                return Err("Invalid byte characters, bytes are A-Z or a-z"); 
            }
        }
        return Ok(ChunkType{
            bytes: str::from_utf8(&value).unwrap().to_string()
        }); 
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for b in s.as_bytes().iter(){
            if (b < &65) || (&90 < b && b < &97) || (&122 < b){
                return Err("Invalid PNG bytes"); 
            }
        }
        return Ok(ChunkType {
            bytes: s.to_owned()
        }); 
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.bytes)
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        for (i, c) in self.bytes.as_bytes().iter().enumerate(){
            if *c != other.bytes.as_bytes()[i]{
                return false; 
            }
        }
        return true; 
    }
}

impl Eq for ChunkType {}


//Tests
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
        println!("{}", chunk); 
        println!("{}", chunk.bytes); 
        println!("{}", chunk.to_string()); 
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