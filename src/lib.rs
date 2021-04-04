//! This crate is a simple XOR based one-time-pad operating on byte vectors

extern crate getrandom;

#[derive(Debug)]
pub struct OneTimePad;

impl OneTimePad {

    /// ```rust
    /// use one_time_pad::OneTimePad;
    /// use std::error::Error;
    ///
    /// fn main() {
    ///     let pad = OneTimePad::generate_random_pad(6).unwrap();
    ///     let encrypted_data = OneTimePad::encrypt(&pad, &vec![1,2,3,4,5,6]);
    ///     println!("Encrypted Data: {:?}", encrypted_data); 
    /// }
    /// ```
    pub fn encrypt(pad_buffer: &Vec<u8>, plain_text_buffer: &Vec<u8>) -> Vec<u8> {
        return operate(pad_buffer, plain_text_buffer);
    }

    /// ```rust
    /// use one_time_pad::OneTimePad;
    /// use std::error::Error;
    ///
    /// fn main() {
    ///     let pad = OneTimePad::generate_random_pad(6).unwrap();
    ///     let encrypted_data = OneTimePad::decrypt(&pad, &vec![1,2,3,4,5,6]);
    ///     println!("Encrypted Data: {:?}", encrypted_data); 
    /// }
    /// ```
    pub fn decrypt(pad_buffer: &Vec<u8>, encrypted_data_buffer: &Vec<u8>) -> Vec<u8> {
        return operate(pad_buffer, encrypted_data_buffer);
    }

    /// ```rust
    /// use one_time_pad::OneTimePad;
    /// use std::error::Error;
    ///
    /// fn main() {
    ///     let pad = OneTimePad::generate_random_pad(6).unwrap();
    ///     println!("Encryption Pad: {:?}", pad); 
    /// }
    /// ```
    pub fn generate_random_pad(length: usize) -> Result<Vec<u8>, getrandom::Error> {
        let mut arr: Vec<u8> = vec![0; length];
        getrandom::getrandom(&mut arr)?;
        // let vec: Vec<i16> = arr.iter().map(|x| *x as i16).collect();
        Ok(arr)
    }

    // pub fn build_pad_from_file(pad_buffer: &Vec<u8>, length: usize) -> Vec<u8> {

    // }
}

fn operate(pad_buffer: &Vec<u8>, data_buffer: &Vec<u8>) -> Vec<u8> {
    error_check(&pad_buffer, &data_buffer);

    let result: Vec<u8> = pad_buffer
        .iter()
        .zip(data_buffer.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();

    return result;
}

fn error_check(buf_a: &Vec<u8>, buf_b: &Vec<u8>) {
    if buf_a.len() != buf_b.len() || buf_a.len() == 0 || buf_b.len() == 0 {
        panic!("Buffer lengths do not match or cannot be zero");
    }
}

#[cfg(test)]
mod tests {

    use crate::OneTimePad;

    fn generate_random_data(length: usize) -> Result<Vec<u8>, getrandom::Error> {
        let mut arr: Vec<u8> = vec![0; length];
        getrandom::getrandom(&mut arr)?;
        // let vec: Vec<i16> = arr.iter().map(|x| *x as i16).collect();
        Ok(arr)
    }

    #[test]
    fn can_load_random_values_into_buffer() {
        let empty = [0u8; 64];

        let res = OneTimePad::generate_random_pad(64);
        let result = res.unwrap();

        assert_ne!(result, empty);
    }

    #[test]
    fn encrypt_not_empty() {
        let plain_text = vec![1, 2, 3, 4, 5, 6, 7];
        let pad = vec![7, 6, 5, 4, 3, 2, 1];

        let encrypted_data = OneTimePad::encrypt(&pad, &plain_text);

        assert_ne!(encrypted_data.len(), 0);
    }

    #[test]
    fn basic_encrypt_then_decrypt() {
        let plain_text = vec![1, 2, 3, 4, 5, 6, 7];
        let pad = vec![7, 6, 5, 4, 3, 2, 1];

        let encrypted_data = OneTimePad::encrypt(&pad, &plain_text);
        let decrypted_data = OneTimePad::decrypt(&pad, &encrypted_data);

        assert_ne!(plain_text, encrypted_data);
        assert_ne!(encrypted_data, decrypted_data);

        assert_eq!(plain_text, decrypted_data);
    }

    #[test]
    fn edge_cases_encrypt() {
        let plain_text = vec![0, 0, 0, 1, 1, 1, 255, 255, 255];
        let pad = vec![0, 1, 255, 0, 1, 255, 0, 1, 255];

        let known_result: Vec<u8> = vec![0, 1, 255, 1, 0, 254, 255, 254, 0];

        let encrypted_data = OneTimePad::encrypt(&pad, &plain_text);

        assert_eq!(known_result, encrypted_data);
    }

    #[test]
    fn small_randomised_encrypt_then_decrypt() {
        let plain_text = generate_random_data(10).unwrap();
        let pad = OneTimePad::generate_random_pad(10).unwrap();

        let encrypted_data = OneTimePad::encrypt(&pad, &plain_text);
        let decrypted_data = OneTimePad::decrypt(&pad, &encrypted_data);

        assert_ne!(plain_text, encrypted_data);
        assert_ne!(encrypted_data, decrypted_data);

        assert_eq!(plain_text, decrypted_data);
    }

    #[test]
    fn large_randomised_encrypt_then_decrypt() {
        let plain_text = generate_random_data(1000).unwrap();
        let pad = OneTimePad::generate_random_pad(1000).unwrap();

        let encrypted_data = OneTimePad::encrypt(&pad, &plain_text);
        let decrypted_data = OneTimePad::decrypt(&pad, &encrypted_data);

        assert_ne!(plain_text, encrypted_data);
        assert_ne!(encrypted_data, decrypted_data);

        assert_eq!(plain_text, decrypted_data);
    }

    #[test]
    fn huge_randomised_encrypt_then_decrypt() {
        let plain_text = generate_random_data(100000).unwrap();
        let pad = OneTimePad::generate_random_pad(100000).unwrap();

        let encrypted_data = OneTimePad::encrypt(&pad, &plain_text);
        let decrypted_data = OneTimePad::decrypt(&pad, &encrypted_data);

        assert_ne!(plain_text, encrypted_data);
        assert_ne!(encrypted_data, decrypted_data);

        assert_eq!(plain_text, decrypted_data);
    }

    #[test]
    fn different_pad_results_in_same_plain_text_when_decrypting() {
        let plain_text = generate_random_data(10).unwrap();

        let pad = OneTimePad::generate_random_pad(10).unwrap();
        let encrypted_data = OneTimePad::encrypt(&pad, &plain_text);
        let decrypted_data = OneTimePad::decrypt(&pad, &encrypted_data);

        let new_pad = OneTimePad::generate_random_pad(10).unwrap();
        let new_encrypted_data = OneTimePad::encrypt(&new_pad, &plain_text);
        let new_decrypted_data = OneTimePad::decrypt(&new_pad, &new_encrypted_data);

        assert_eq!(decrypted_data, new_decrypted_data);
    }

    #[test]
    fn large_different_pad_results_in_same_plain_text_when_decrypting() {
        let plain_text = generate_random_data(1000).unwrap();

        let pad = OneTimePad::generate_random_pad(1000).unwrap();
        let encrypted_data = OneTimePad::encrypt(&pad, &plain_text);
        let decrypted_data = OneTimePad::decrypt(&pad, &encrypted_data);

        let new_pad = OneTimePad::generate_random_pad(1000).unwrap();
        let new_encrypted_data = OneTimePad::encrypt(&new_pad, &plain_text);
        let new_decrypted_data = OneTimePad::decrypt(&new_pad, &new_encrypted_data);

        assert_eq!(decrypted_data, new_decrypted_data);
    }
}
