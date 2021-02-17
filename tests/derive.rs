//! Tests for custom derive support

#![cfg(feature = "derive")]

use simple_tlv::{Decodable, Encodable, Encoder, UntaggedCollection};
// use hex_literal::hex;

#[derive(Clone, Copy, Debug, Eq, PartialEq, UntaggedCollection)]
#[tlv(tag = "0xAA")]
struct S {
    #[tlv(tag = "0x11")]
    x: [u8; 2],
    #[tlv(tag = "0x22")]
    y: [u8; 3],
    #[tlv(tag = "0x33")]
    z: [u8; 4],
}

#[test]
fn derived_reconstruct() {
    let s = S { x: [1,2], y: [3,4,5], z: [6,7,8,9] };
    let mut buf = [0u8; 1024];

    let encoded = s.encode_to_slice(&mut buf).unwrap();

    assert_eq!(encoded,
        &[0xAA, 15,
            0x11, 2, 1, 2,
            0x22, 3, 3, 4, 5,
            0x33, 4, 6, 7, 8, 9,
        ],
    );

    let s2 = S::from_bytes(encoded).unwrap();

    assert_eq!(s, s2);
}
