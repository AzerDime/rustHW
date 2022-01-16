//!Homework 2 for CS410P - Rust
//! Ian Guy 2022

use toyrsa::*;

#[test]
fn test_toyrsa(){
    let edgeMsgZero = 0;
    let edgeMsgOne = 1;
    let pKeyGiven = 0xed23e6cd;
    let qKeyGiven = 0xf050a04d;
    let pubKeyGiven = 0xde9c5816141c8ba9;
    let msgGiven = 0x12345f;

    //Verifies the example test case given with the assignment.
    //Also verifies if encrypted message is actually different than the provided message.
    assert_eq!(0x6418280e0c4d7675, encrypt(pubKeyGiven, msgGiven));
    assert_eq!(0x12345f, decrypt((pKeyGiven, qKeyGiven), msgGiven));

    //Using known valid p and q keys from previous case, check if edge cases for encrypting 0 and 1 hold true.
    assert_eq!(0, encrypt(pubKeyGiven, edgeMsgZero));
    assert_eq!(1, encrypt(pubKeyGiven, edgeMsgOne));



}