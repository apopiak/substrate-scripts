#[allow(unused)]
use parity_scale_codec::{Encode, Decode};
#[allow(unused)]
use sp_io::hashing::{blake2_128, blake2_256, twox_64, twox_128, twox_256};

fn main() {
    let bytes = hex::decode("003665c4ffffffff").unwrap();
    let res = i64::decode(&mut &bytes[..]);
    println!("{:?}", res);

    let module = b"Balances";
    let hash = twox_128(module);
    println!("{}", hex::encode(hash));
}
