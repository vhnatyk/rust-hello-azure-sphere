/*
    Multi-party ECDSA

    Copyright 2018 by Kzen Networks

    This file is part of Multi-party ECDSA library
    (https://github.com/KZen-networks/multi-party-ecdsa)

    Multi-party ECDSA is free software: you can redistribute
    it and/or modify it under the terms of the GNU General Public
    License as published by the Free Software Foundation, either
    version 3 of the License, or (at your option) any later version.

    @license GPL-3.0+ <https://github.com/KZen-networks/multi-party-ecdsa/blob/master/LICENSE>
*/
//#![feature(lang_items, start, libc)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate rand;
extern crate zeroize;

extern crate cryptoxide;

pub mod curv;
pub mod gg_2018;
pub mod paillier;

#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum Error {
    InvalidKey,
    InvalidSS,
    InvalidCom,
    InvalidSig,
}
#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum ErrorKey {
    InvalidPublicKey,
}

pub enum ErrorSS {
    VerifyShareError,
}

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;

#[cfg(all(test, target_arch = "wasm32"))]
extern crate wasm_bindgen_test;
