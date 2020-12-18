// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(non_snake_case)]

macro_rules! impl_aes {
    ($impl:ident, $key_len:expr, $iv_len:expr, $tag_len:expr $(,)?) => {
        pub const KEY_LENGTH: usize = $key_len;
        pub const IV_LENGTH: usize = $iv_len;
        pub const TAG_LENGTH: usize = $tag_len;

        pub fn encrypt(
            key: &[u8; KEY_LENGTH],
            iv: &[u8; IV_LENGTH],
            associated_data: &[u8],
            plaintext: &[u8],
            ciphertext: &mut [u8],
            tag: &mut [u8; TAG_LENGTH],
        ) -> $crate::Result<()> {
            use chacha20poly1305::aead::{AeadMutInPlace as _, NewAead as _};

            if plaintext.len() > ciphertext.len() {
                return Err($crate::Error::BufferSize {
                    needs: plaintext.len(),
                    has: ciphertext.len(),
                });
            }

            ciphertext.copy_from_slice(plaintext);

            let t = $impl::new(key.into())
                .encrypt_in_place_detached((iv as &[_]).into(), associated_data, ciphertext)
                .map_err(|_| $crate::Error::CipherError {
                    alg: concat!(stringify!($impl), "::encrypt"),
                })?;

            tag.copy_from_slice(&t);

            Ok(())
        }

        pub fn decrypt(
            key: &[u8; KEY_LENGTH],
            iv: &[u8; IV_LENGTH],
            associated_data: &[u8],
            tag: &[u8; TAG_LENGTH],
            ciphertext: &[u8],
            plaintext: &mut [u8],
        ) -> $crate::Result<()> {
            use chacha20poly1305::aead::{AeadMutInPlace as _, NewAead as _};

            if ciphertext.len() > plaintext.len() {
                return Err($crate::Error::BufferSize {
                    needs: ciphertext.len(),
                    has: plaintext.len(),
                });
            }

            plaintext.copy_from_slice(ciphertext);

            $impl::new(key.into())
                .decrypt_in_place_detached((iv as &[_]).into(), associated_data, plaintext, tag.into())
                .map_err(|_| $crate::Error::CipherError {
                    alg: concat!(stringify!($impl), "::decrypt"),
                })
        }
    };
}

pub mod XCHACHA20_POLY1305 {
    use chacha20poly1305::XChaCha20Poly1305;

    impl_aes!(XChaCha20Poly1305, /* key */ 32, /* iv */ 24, /* tag */ 16);

    // Defined for backwards compatability
    pub const XCHACHA20POLY1305_KEY_SIZE: usize = self::KEY_LENGTH;

    // Defined for backwards compatability
    pub const XCHACHA20POLY1305_NONCE_SIZE: usize = self::IV_LENGTH;

    // Defined for backwards compatability
    pub const XCHACHA20POLY1305_TAG_SIZE: usize = self::TAG_LENGTH;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestVector {
        plaintext: &'static str,
        associated_data: &'static str,
        key: &'static str,
        nonce: &'static str,
        ciphertext: &'static str,
        tag: &'static str,
    }

    #[test]
    fn test_vectors_XCHACHA20_POLY1305() -> crate::Result<()> {
        let tvs = [
            // https://tools.ietf.org/html/draft-arciszewski-xchacha-03#appendix-A.3
            TestVector {
                plaintext: "4c616469657320616e642047656e746c656d656e206f662074686520636c617373206f66202739393a204966204920636f756c64206f6666657220796f75206f6e6c79206f6e652074697020666f7220746865206675747572652c2073756e73637265656e20776f756c642062652069742e",
                associated_data: "50515253c0c1c2c3c4c5c6c7",
                key: "808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f",
                nonce: "404142434445464748494a4b4c4d4e4f5051525354555657",
                ciphertext: "bd6d179d3e83d43b9576579493c0e939572a1700252bfaccbed2902c21396cbb731c7f1b0b4aa6440bf3a82f4eda7e39ae64c6708c54c216cb96b72e1213b4522f8c9ba40db5d945b11b69b982c1bb9e3f3fac2bc369488f76b2383565d3fff921f9664c97637da9768812f615c68b13b52e",
                //poly1305_key = "7b191f80f361f099094f6f4b8fb97df847cc6873a8f2b190dd73807183f907d5",
                tag: "c0875924c1c7987947deafd8780acf49",
            },

            // generated using: utils/test_vectors/py/main.py
            TestVector {
                plaintext: "90fe4f823a01ec0caa5a9f6199925440c209549c16193834f49f6a710b18eabbde04379e41afb4b9cec0d5f103ad9a2ecde7372bb10482b6871a57b994ab85bb3c3447eb11c5aa3fba2866cfc9d46e6e6a56e1b40f4db48180fbffd130d0b9dc84e5182bdd62a57a2e48002d984e",
                associated_data: "",
                key: "a4840111fcef9004e37fc9782e11fedb61139f2e9770b2ed4cefa0968902dbd1",
                nonce: "e91fa29325eccc3f89a81608bf221f9716fbc71e9c44f042",
                ciphertext: "5711753f5adf79c61a904bb868191446c7af3f70356b577f95b7fe977e75bf422ae6bdd0f38e1da30ed197d007ed02ca927bf49bc450a1a1010ddf372955b0e88731b152a8f31f375a0941c7dfb2b1b4ed685e1f646af09f4df94992f3d7736ca2504c53c59a6a120c52722fb487",
                tag: "6a61f872f120375fcbb71fadfed30a25",
            },
            TestVector {
                plaintext: "",
                associated_data: "",
                key: "e2d9b9d1741c68f46f659b3d3d13a75065f5ea6520420089777a4e7958a41375",
                nonce: "46db74642b7129f0dc7c0b8fcea644505dd2d1ed91312ee7",
                ciphertext: "",
                tag: "bf9dc5041afad04421dffd32f9d7c5bb",
            },
            TestVector {
                plaintext: "1077d20d37b78c1a7081f2a820fac12ead26d49e5dcfc7661d5db78a9f297c6b8ece1fcbea535b4fad1770646783f44e57b93d9570e6f7de7fc9192e5711a7a95c548adbd3b522b2772b61a5c2e5f4039d8ddaacd4932a7359b307d1d232fd76375ae8cf027e3ef3cc91aa683171b4e2f3f2c643aa9be199b2625a68bd5665720d110deeaf43",
                associated_data: "3697af02439801d4d8086e36deb9692e1d122cff0f639e15f022fbb46fc15250f4c8aa87f111816b0b5f3e148e38332ea7b114efd46f38c4b1520b8cb38923f0c3bb38cf03831ca3057394a4f42043f32afbaf0eaefc508f8056f0513d3c5ab20434b54f8002337b23d2b0026326d9ffdf",
                key: "f44b5f74905f33c8e2d5ac6b2ac09c2767123728fccaa2c5f17c795dd21b60f4",
                nonce: "7af6a1eaf134bfde94bb49abf680a782dd52aa060a77bda1",
                ciphertext: "50df21404281883cb14dab5a397eaeb81f859bda458482f31fc5586c37257ae249fd75d4ee2e1aaa74f7a123201e03c584e8ac9b21383ebd84e3b1cba68f1590f4a8752d8ec8d5540a44952922eb2679744ba087bf3672653fa5649d22a4a3b27e91a9f9693266611a65745167306ae5d7c16298e4323fbfc9c0d5cd1be9d7b9e234648d8c14",
                tag: "418d7e4c54ac8263bec3a9326f336f21",
            },
        ];

        for tv in tvs.iter() {
            let plaintext = hex::decode(tv.plaintext).unwrap();
            let associated_data = hex::decode(tv.associated_data).unwrap();

            let mut key = [0; XCHACHA20_POLY1305::KEY_LENGTH];
            hex::decode_to_slice(tv.key, &mut key).unwrap();
            let mut nonce = [0; XCHACHA20_POLY1305::IV_LENGTH];
            hex::decode_to_slice(tv.nonce, &mut nonce).unwrap();

            let expected_ciphertext = hex::decode(tv.ciphertext).unwrap();
            let expected_tag = hex::decode(tv.tag).unwrap();

            let mut ciphertext = vec![0; plaintext.len()];
            let mut tag = [0; XCHACHA20_POLY1305::TAG_LENGTH];

            XCHACHA20_POLY1305::encrypt(
                &key,
                &nonce,
                &associated_data,
                &plaintext,
                &mut ciphertext,
                &mut tag,
            )?;

            assert_eq!(expected_ciphertext, ciphertext);
            assert_eq!(expected_tag, tag);

            let mut decrypted_plain_text = vec![0; ciphertext.len()];
            XCHACHA20_POLY1305::decrypt(
                &key,
                &nonce,
                &associated_data,
                &tag,
                &ciphertext,
                &mut decrypted_plain_text,
            )?;
            assert_eq!(decrypted_plain_text, plaintext);

            let mut corrupted_tag = tag;
            crate::test_utils::corrupt(&mut corrupted_tag);
            assert!(XCHACHA20_POLY1305::decrypt(
                &key,
                &nonce,
                &associated_data,
                &corrupted_tag,
                &ciphertext,
                &mut decrypted_plain_text,
            )
            .is_err());

            let mut corrupted_nonce = nonce;
            crate::test_utils::corrupt(&mut corrupted_nonce);
            assert!(XCHACHA20_POLY1305::decrypt(
                &key,
                &corrupted_nonce,
                &associated_data,
                &tag,
                &ciphertext,
                &mut decrypted_plain_text,
            )
            .is_err());

            if !associated_data.is_empty() {
                let mut corrupted_associated_data = associated_data.clone();
                crate::test_utils::corrupt(&mut corrupted_associated_data);
                assert!(XCHACHA20_POLY1305::decrypt(
                    &key,
                    &nonce,
                    &corrupted_associated_data,
                    &tag,
                    &ciphertext,
                    &mut decrypted_plain_text,
                )
                .is_err());
                assert!(XCHACHA20_POLY1305::decrypt(
                    &key,
                    &nonce,
                    &crate::test_utils::fresh::bytestring(),
                    &tag,
                    &ciphertext,
                    &mut decrypted_plain_text,
                )
                .is_err());
            } else {
                assert!(XCHACHA20_POLY1305::decrypt(
                    &key,
                    &nonce,
                    &crate::test_utils::fresh::non_empty_bytestring(),
                    &tag,
                    &ciphertext,
                    &mut decrypted_plain_text,
                )
                .is_err());
            }
        }

        Ok(())
    }
}
