/*
 * Copyright (c) 2012-2020 MIRACL UK Ltd.
 *
 * This file is part of MIRACL Core
 * (see https://github.com/miracl/core).
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![cfg_attr(not(feature = "std"), no_std)]

#![allow(clippy::many_single_char_names)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::new_without_default)]

pub mod arch;
pub mod aes;
pub mod gcm;
pub mod hmac;
pub mod hash256;
pub mod hash384;
pub mod hash512;
pub mod rand;
pub mod share;
pub mod sha3;
pub mod nhs;
pub mod x509;
pub mod bls12381;

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
pub fn bls_init() -> isize {
    bls12381::bls::init()
}

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
pub fn bls_verify(sig: &[u8], m: &[u8], s: &[u8]) -> isize {
    bls12381::bls::core_verify(sig, m, s)
}

#[cfg(test)]
mod tests {
    #[cfg(all(feature = "allow_alt_compress", not(feature = "fallback_separator")))]
    #[test]
    fn bls_verify() {
        use hex_literal::hex;
        use crate::bls12381::bls::{core_verify, init, BLS_FAIL, BLS_OK};
        let pk = hex!("a7623a93cdb56c4d23d99c14216afaab3dfd6d4f9eb3db23d038280b6d5cb2caaee2a19dd92c9df7001dede23bf036bc0f33982dfb41e8fa9b8e96b5dc3e83d55ca4dd146c7eb2e8b6859cb5a5db815db86810b8d12cee1588b5dbf34a4dc9a5");
        let sig = hex!("b89e13a212c830586eaa9ad53946cd968718ebecc27eda849d9232673dcd4f440e8b5df39bf14a88048c15e16cbcaabe");
        assert_eq!(init(), BLS_OK);
        assert_eq!(core_verify(&sig, b"hello".as_ref(), &pk), BLS_OK);
        assert_eq!(core_verify(&sig, b"hallo".as_ref(), &pk), BLS_FAIL);
    }
}