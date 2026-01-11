use heapless::{String, Vec};

#[inline(never)]
pub fn vec_to_string_new(vec: Vec<u8, 20>) -> Result<String<20>, Vec<u8, 20>> {
    match String::from_utf8(vec) {
        Ok(string) => Ok(string),
        Err(err) => Err(err.into_bytes()),
    }
}

#[inline(never)]
pub fn vec_to_string_old(vec: Vec<u8, 20>) -> Result<String<20>, Vec<u8, 20>> {
    let vec_clone = vec.clone();
    match String::from_utf8_old(vec) {
        Ok(string) => Ok(string),
        Err(_err) => Err(vec_clone),
    }
}

pub fn main() {
    let vec = Vec::<u8, 20>::new();
    let _ = vec_to_string_new(vec.clone());
    let _ = vec_to_string_old(vec.clone());
}
