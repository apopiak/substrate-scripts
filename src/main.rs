// #[allow(unused)]
// use parity_scale_codec::{Encode, Decode};
#[allow(unused)]
use sp_io::hashing::{blake2_128, blake2_256, twox_64, twox_128, twox_256};

fn main() {
    // let bytes = hex::decode("003665c4ffffffff").unwrap();
    // let res = i64::decode(&mut &bytes[..]);
    // println!("{:?}", res);

    // let module = b"Balances";
    // let hash = twox_128(module);
    // println!("{}, length: {}", hex::encode(hash), hex::encode(hash).len());

    let modules: Vec<Vec<u8>> = vec![
        b"System".to_vec(),
        b"RandomnessCollectiveFlip".to_vec(),
        b"Scheduler".to_vec(),
        b"Babe".to_vec(),
        b"Timestamp".to_vec(),
        b"Indices".to_vec(),
        b"Balances".to_vec(),
        b"TransactionPayment".to_vec(),
        b"Authorship".to_vec(),
        b"Staking".to_vec(),
        b"Offences".to_vec(),
        b"Historical".to_vec(),
        b"Session".to_vec(),
        b"FinalityTracker".to_vec(),
        b"Grandpa".to_vec(),
        b"ImOnline".to_vec(),
        b"AuthorityDiscovery".to_vec(),
        b"Democracy".to_vec(),
        b"Council".to_vec(),
        b"TechnicalCommittee".to_vec(),
        b"ElectionsPhragmen".to_vec(),
        b"TechnicalMembership".to_vec(),
        b"Treasury".to_vec(),
        b"Claims".to_vec(),
        b"Vesting".to_vec(),
        b"Utility".to_vec(),
        b"Identity".to_vec(),
        b"Proxy".to_vec(),
        b"Multisig".to_vec(),
    ];


    let raw_ident = vec![38, 170, 57, 78, 234, 86, 48, 224, 124, 72, 174, 12, 149, 88, 206, 247, 185, 157, 136, 14, 198, 129, 121, 156, 12, 243, 14, 136, 134, 55, 29, 169, 64, 107, 148, 77, 133, 20, 121, 136, 152, 50, 179, 6, 20, 110, 233, 189, 38, 19, 51, 83, 33, 61, 117, 77, 0, 132, 244, 130, 106, 59, 31, 106, 129, 76, 60, 90, 99, 152, 73, 11, 185, 130, 2, 201, 212, 205, 140, 207];
    let hex_str = hex::encode(raw_ident);
    println!("{}", hex_str);
    for module in modules {
        let hash = twox_128(&module);
        let module_hex = hex::encode(hash);
        println!("{}, match: {} (module: {})", module_hex, hex_str.starts_with(&module_hex), String::from_utf8(module).unwrap());
    }

    // let bytes = hex::decode("1809d78346727a0ef58c0fa03bafa3231d885dcfb277f185f2d8e62a5f290c855bd6a3e1e8406faa3c82ab06b794c99f14a161973be7aa6012568b1c491d45ec969ed7420bcfaa59").unwrap();
    // println!("{:?}", bytes);


    // let bytes = vec![50, 0, 0, 0, 2, 92, 220, 105, 60, 210, 167, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 125, 181, 42, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 252, 98, 249, 61, 154, 60, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 252, 98, 249, 61, 154, 60, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // let bytes = vec![65, 108, 105, 99, 101];
    // println!("{}", hex::encode(bytes));

}
