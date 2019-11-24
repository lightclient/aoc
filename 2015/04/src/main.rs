use md5;
use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // the mask is for the upper 5 nibbles
    let mask = u128::from_le_bytes([255, 255, 240, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let (five_nonce, five_hash) = find_hash(&input, mask);

    // the mask is for the upper 6 nibbles
    let mask = u128::from_le_bytes([255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let (six_nonce, six_hash) = find_hash(&input, mask);

    println!(
        "The hash for input {} for difficulty 5 is {:x} using nonce {}",
        input, five_hash, five_nonce
    );

    println!(
        "The hash for input {} for difficulty 6 is {:x} using nonce {}",
        input, six_hash, six_nonce
    );

    Ok(())
}

fn find_hash(s: &String, mask: u128) -> (u32, u128) {
    let mut nonce = 0;
    let mut hash = std::u128::MAX;

    while hash & mask != 0 {
        nonce += 1;
        hash = u128::from_le_bytes(md5::compute(combine_with_nonce(s, nonce)).0);
    }

    (nonce, hash)
}

fn combine_with_nonce(s: &String, nonce: u32) -> Vec<u8> {
    format!("{}{}", s, nonce.to_string()).as_bytes().to_vec()
}
