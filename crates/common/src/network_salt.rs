// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

//
//
// todo: make this a hash of the network id so it is unique per p2p network

pub static NET_SALT: [u8; 32] = [
    0x4b, 0x66, 0xe9, 0xd4, 0xd1, 0xb4, 0x67, 0x3c, 0x5a, 0xd2, 0x26, 0x91, 0x95, 0x7d, 0x6a, 0xf5,
    0xc1, 0x1b, 0x64, 0x21, 0xe0, 0xea, 0x01, 0xd4, 0x2c, 0xa4, 0x16, 0x9e, 0x79, 0x18, 0xba, 0x0d,
];

// IV for AES w/o hamc
pub static AES_IV: [u8; 16] = [
    0x1e, 0xff, 0x01, 0x32, 0xad, 0xfa, 0x24, 0x55, 0xe1, 0x94, 0x8d, 0x57, 0x3c, 0xaa, 0xf5, 0x82,
];