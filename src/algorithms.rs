use md5;
use sha2::{Digest, Sha256};

pub fn calc_sha256(filepath: &str) -> String {
    //println!("{}", filepath);
    
    let file_as_bytes = std::fs::read(filepath);
    let hash = Sha256::digest(file_as_bytes.unwrap());

    format!("{:x}", hash)
}

pub fn calc_md5(filepath: &str) -> String {
    let file_as_bytes = std::fs::read(filepath).unwrap();
    let hash = md5::compute(file_as_bytes);

    format!("{:x}", hash)
}
