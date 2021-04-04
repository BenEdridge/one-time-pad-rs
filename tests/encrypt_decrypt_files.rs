use one_time_pad::OneTimePad;
use std::io::Read;
use std::{fs::File, io::Write};

#[test]
fn read_example_file_encrypt_write_then_decrypt() {

    let mut file = File::open("example.txt").unwrap();
    let mut encrypted_file = File::create("example.txt.encrypted").unwrap();

    let mut plain_text = Vec::new();
    file.read_to_end(&mut plain_text).unwrap();

    let pad = OneTimePad::generate_random_pad(plain_text.len()).unwrap();
    let encrypted_data = OneTimePad::encrypt(&pad, &plain_text);
    encrypted_file.write(&encrypted_data).unwrap();

    let mut encrypted_text = Vec::new();
    let mut encrypted_file = File::open("example.txt.encrypted").unwrap();
    encrypted_file.read_to_end(&mut encrypted_text).unwrap();

    let decrypted_data = OneTimePad::decrypt(&pad, &encrypted_text);

    assert_eq!(plain_text, decrypted_data);
}
