use super::*;

// use e_api::chacha::ChaCha;
// use aes::cipher::block_padding::ZeroPadding;
// use aes::cipher::generic_array::GenericArray;
// use aes::cipher::typenum::Zero;
use std::error::Error;

pub enum Tgz_Err {
    Could_Not_Get_Entries,
    Could_Not_Get_One_Entry,
    Could_Not_Read_Entry,
}

pub struct Tgz_Util {}

impl Tgz_Util {
    pub fn gunzip(data: &[u8]) -> Res<Vec<u8>> {
        use flate2::read::GzDecoder;
        let mut gz = GzDecoder::new(data);
        let mut out: Vec<u8> = Vec::new();
        let _ = gz.read_to_end(&mut out).map_err(|_| ());
        Ok(out)
    }

    /*
    pub fn bunzip2(data: &[u8]) -> Res<Vec<u8>> {
        use bzip2_rs::DecoderReader;
        use std::fs::File;
        use std::io;

        let mut reader = DecoderReader::new(data);
        let mut out = vec![];
        match reader.read_to_end(&mut out) {
            Ok(_) => Ok(out),
            Err(_) => {
                damn_it!("")
            }
        }
    }*/

    /*
    pub fn xuz(data: &[u8]) -> Res<Vec<u8>> {
        let mut f = std::io::BufReader::new(data);
        let mut decomp: Vec<u8> = Vec::new();
        match lzma_rs::xz_decompress(&mut f, &mut decomp) {
            Ok(_) => Ok(decomp),
            Err(_) => {
                damn_it!("")
            }
        }
    }

     */

    /*
    pub fn gzip(data: &[u8]) -> Vec<u8> {
        use flate2::read::GzEncoder;
        let mut gz = GzEncoder::new(data, Compression::best());
        let mut out: Vec<u8> = Vec::new();
        gz.read_to_end(&mut out).unwrap();
        out
    }

    pub fn untar_raw(data: &[u8]) -> Res<FlatFS> {
        let mut ans = HashMap::new();

        let mut ar = tar::Archive::new(data);

        for e in ar.entries().map_err(|_| err!("Untar Raw"))? {
            let mut e = e.map_err(|_| err!("Untar Raw"))?;
            let mut file_content = vec![];
            e.read_to_end(&mut file_content)
                .map_err(|_| err!("Untar Raw"))?;
            let fname =
                String::from_utf8_lossy(e.path_bytes().as_ref()).into_owned();
            ans.insert(fname, Arc::new(file_content));
        }

        Ok(FlatFS { inner: ans })
    }

    pub fn untar_gz(data: &[u8]) -> Res<FlatFS> {
        let data = Self::gunzip(data)?;
        Self::untar_raw(data.as_ref())
    }

    pub fn untar_xz(data: &[u8]) -> Res<FlatFS> {
        let data = Self::xuz(data)?;
        Self::untar_raw(data.as_ref())
    }

     */

    /*
    pub fn untar_bz2(data: &[u8]) -> Res<FlatFS> {
        let data = Self::bunzip2(data)?;
        Self::untar_raw(data.as_ref())
    }
    */

    /*
    pub fn decrypt_aes(_f: &[u8]) -> Res<Vec<u8>> {
        damn_it!("")
        /*
        use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyInit, KeyIvInit};

        let key = hex::decode("436f7079726967687465642e20446f206e6f74207573652f73686172652e200a").map_err(|e| err!("Key Error: {:?}", e))?;
        let iv = hex::decode("427579207572206c6963656e73652e0a").map_err(|e| err!("Iv Error: {:?}", e))?;

        let decryptor = cbc::Decryptor::<aes::Aes256>::new_from_slices(key.as_slice(), iv.as_slice()).map_err(|e| err!("Decrypto Error: {:?}", e))?;
        let plaintext = decryptor.decrypt_padded_vec_mut::<Pkcs7>(f).map_err(|e| err!("decrypt error: {:?}", e))?;
        Ok(plaintext)
        */
    }


    pub fn decrypt_chacha(f: &[u8]) -> Res<Vec<u8>> {
        // let key = [0_u8; 32];
        let iv = [0_u8; 12];

        let key = hex::decode(
            "436f7079726967687465642e20446f206e6f74207573652f73686172652e200a",
        )
        .map_err(|e| err!("Key Error: {:?}", e))?;
        // let iv = hex::decode("427579207572206c6963656e").map_err(|e| err!("Iv Error: {:?}", e))?;

        let mut cipher = ChaCha20::new(
            (&key[0..32]).try_into().unwrap(),
            (&iv[0..12]).try_into().unwrap(),
        );

        let mut out = f.to_vec();
        cipher.apply_keystream(&mut out);

        Ok(out)

        /*
        let mut buffer = plaintext.clone();

        // apply keystream (encrypt)
        cipher.apply_keystream(&mut buffer);
        assert_eq!(buffer, ciphertext);

        let ciphertext = buffer.clone();

        // ChaCha ciphers support seeking
        cipher.seek(0u32);

        // decrypt ciphertext by applying keystream again
        cipher.apply_keystream(&mut buffer);
        assert_eq!(buffer, plaintext);

        // stream ciphers can be used with streaming messages
        cipher.seek(0u32);
        for chunk in buffer.chunks_mut(3) {
            cipher.apply_keystream(chunk);
        }
        assert_eq!(buffer, ciphertext);

             */
    }

    pub fn pbkdf2(_f: &[u8]) -> Vec<u8> {
        damn_it!("")
        /*
        use pbkdf2::{pbkdf2_hmac, pbkdf2_hmac_array};
        use sha2::Sha256;

        fn derive_key(password: &[u8], iterations: u32, output_length: usize) -> Vec<u8> {
            let mut key = vec![0u8; output_length];
            pbkdf2_hmac::<Sha256>(password, b"", iterations, &mut key);
            key
        }

        // Usage example
        let iterations = 10_000;
        let output_length = 32; // 256-bit key

        let derived_key = derive_key(f, iterations, output_length);
        println!("Derived key: {:?}", derived_key);

        derived_key

         */
    }
    */
}
