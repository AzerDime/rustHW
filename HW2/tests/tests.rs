//!Homework 2 for CS410P - Rust
//! Ian Guy 2022

use rand::prelude::*;
use toyrsa::*;

#[test]
fn test_toyrsa() {
    let edge_msg_zero = 0;
    let edge_msg_one = 1;
    let p_key_given = 0xed23e6cd;
    let q_key_given = 0xf050a04d;
    let pub_key_given = 0xde9c5816141c8ba9;
    let encrypted_given = 0x6418280e0c4d7675;
    let msg_given = 0x12345f;

    //Verifies the example test case given with the assignment.
    //Also verifies if encrypted message is actually different than the provided message.
    assert_eq!(encrypted_given, encrypt(pub_key_given, msg_given));
    assert_eq!(
        0x12345f,
        decrypt((p_key_given, q_key_given), encrypted_given)
    );

    //Using known valid p and q keys from previous case, check if edge cases for encrypting 0 and 1 hold true.
    assert_eq!(0, encrypt(pub_key_given, edge_msg_zero));
    assert_eq!(1, encrypt(pub_key_given, edge_msg_one));

    //Declare an iterator and a value to set how many times we want to test this.
    let mut iterate = 0;
    let cases = 5000;

    while iterate < cases {
        let (p, q) = genkey();
        let pubkey = u64::from(p) * u64::from(q);
        let message_test: u32 = random();
        let encrypted_msg = encrypt(pubkey, message_test);
        //If you want to use -- --nocapture...
        if iterate % 1000 == 0{
            println!("Public key, the generated message, and the encrypted message:");
            println!("{},{},{},\n",pubkey,message_test,encrypted_msg);
        }
        assert_eq!(message_test, decrypt((p, q), encrypted_msg));
        iterate += 1;
    }
}
