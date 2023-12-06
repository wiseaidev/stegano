use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;
use std::mem;
/// Performs XOR encrypting or decrypting on the provided byte slice using the specified key.
///
/// # Arguments
///
/// * `input` - The input byte slice to be encrypted or decrypted.
/// * `key` - The key used for XOR encrypting or decrypting. It is repeated cyclically if shorter than the input.
///
/// # Returns
///
/// A `Vec<u8>` containing the result of the XOR operation.
///
/// # Examples
///
/// ```
/// use stegano::utils::xor_encrypt_decrypt;
///
/// let input = b"Hello, World!";
/// let key = "secret_key";
/// let encrypted = xor_encrypt_decrypt(input, key);
/// let decrypted = xor_encrypt_decrypt(&encrypted, key);
/// assert_eq!(input, decrypted.as_slice());
/// ```
pub fn xor_encrypt_decrypt(input: &[u8], key: &str) -> Vec<u8> {
    let mut b_arr = Vec::with_capacity(input.len());
    for (i, &byte) in input.iter().enumerate() {
        b_arr.push(byte ^ key.as_bytes()[i % key.len()]);
    }
    b_arr
}

/// Converts a 64-bit unsigned integer to an array of 8 bytes.
///
/// # Arguments
///
/// * `value` - The 64-bit unsigned integer to be converted.
///
/// # Returns
///
/// An array of 8 bytes representing the input value.
///
/// # Examples
///
/// ```
/// use stegano::utils::u64_to_u8_array;
///
/// let value = 1234567890u64;
/// let byte_array = u64_to_u8_array(value);
/// assert_eq!(value.to_ne_bytes(), byte_array);
/// ```
pub fn u64_to_u8_array(value: u64) -> [u8; 8] {
    let bytes = value.to_ne_bytes();
    let mut _result = [0; 8];

    unsafe {
        // Transmute the byte array into an array of unsigned 8-bit integers
        _result = mem::transmute_copy(&bytes);
    }

    _result
}

/// Prints a hexadecimal representation of the input data with ASCII interpretation.
///
/// # Arguments
///
/// * `data` - A slice of u8 representing the data to be printed.
/// * `offset` - An offset value to be added to the printed hexadecimal addresses.
///
/// # Examples
///
/// ```rust
/// use stegano::utils::print_hex;
///
/// let my_data: Vec<u8> = (0..100).collect();
/// let my_offset: u64 = 0;
/// print_hex(&my_data, my_offset);
/// ```
///
/// # Output
///
/// The function prints the hexadecimal representation of the input data in chunks of 20 bytes.
/// Each chunk is displayed with an address offset, hexadecimal values, ASCII interpretation,
/// and alternating colors (blue and green) for better visibility.
///
/// Hexadecimal values are printed in the following format:
///
/// 00000000 | \x1b[94m00\x1b[0m \x1b[92m01\x1b[0m \x1b[94m02\x1b[0m \x1b[92m03\x1b[0m \x1b[94m04\x1b[0m ... | ........ ........ ........ ........
/// 00000014 | \x1b[94m14\x1b[0m \x1b[92m15\x1b[0m \x1b[94m16\x1b[0m \x1b[92m17\x1b[0m \x1b[94m18\x1b[0m ... | ........ ........ ........ ........
/// ...
///
/// The ASCII interpretation is displayed on the right, and non-printable ASCII characters
/// are represented as dots ('.').
pub fn print_hex(data: &[u8], offset: u64) {
    for (i, chunk) in data.chunks(20).enumerate() {
        print!("{:08} | ", offset + 20 * i as u64);

        for (j, &byte) in chunk.iter().enumerate() {
            // Alternating colors (blue and green)
            let color = if j % 2 == 0 { "\x1b[94m" } else { "\x1b[92m" };
            print!("{}{:02X} \x1b[0m", color, byte);
        }

        print!("| ");

        for byte_chunk in chunk.chunks(4) {
            for byte in byte_chunk {
                print!(
                    "{}",
                    if byte.is_ascii() && byte.is_ascii_graphic() {
                        *byte as char
                    } else {
                        '.'
                    }
                );
            }
        }
        println!();
    }
}

/// Pad the input slice with zeros to create a fixed-size array of 16 bytes.
///
/// # Arguments
///
/// * `slice` - A slice of u8 to be padded with zeros.
///
/// # Returns
///
/// A fixed-size array of 16 bytes containing the original slice content with zero-padding.
///
/// # Examples
///
/// ```
/// use stegano::utils::pad_with_zeros;
///
/// let input_slice: &[u8] = &[1, 2, 3, 4, 5];
/// let padded_array: [u8; 16] = pad_with_zeros(input_slice);
/// assert_eq!(padded_array, [1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// ```
pub fn pad_with_zeros(slice: &[u8]) -> [u8; 16] {
    let mut padded_array: [u8; 16] = [0; 16];
    let len = std::cmp::min(slice.len(), padded_array.len());
    padded_array[..len].copy_from_slice(&slice[..len]);
    padded_array
}

/// Encrypts the payload using AES-128 encryption algorithm with zero-padding.
///
/// # Arguments
///
/// * `key` - A string representing the encryption key.
/// * `payload` - A string representing the payload to be encrypted.
///
/// # Returns
///
/// A vector of u8 containing the encrypted payload.
///
/// # Examples
///
/// ```
/// use stegano::utils::encrypt_payload;
///
/// let key = "secret_key";
/// let payload = "confidential_data";
/// let encrypted_data = encrypt_payload(key, payload);
/// assert_eq!(encrypted_data.len(), 16);
/// ```
pub fn encrypt_payload(key: &str, payload: &str) -> Vec<u8> {
    let in_key: &[u8; 16] = &pad_with_zeros(key.as_bytes());
    let key = GenericArray::clone_from_slice(in_key);

    if payload.len() <= 16 {
        let in_payload: &[u8; 16] = &pad_with_zeros(payload.as_bytes());
        let mut block = GenericArray::clone_from_slice(in_payload);

        let cipher = Aes128::new(&key);
        cipher.encrypt_block(&mut block);
        block.to_vec()
    } else {
        let mut encrypted_data: Vec<u8> = Vec::new();

        for (i, chunk) in payload.as_bytes().chunks_exact(16).enumerate() {
            let in_payload: &[u8; 16] = &pad_with_zeros(chunk);
            let mut block = GenericArray::clone_from_slice(in_payload);

            let cipher = Aes128::new(&key);
            cipher.encrypt_block(&mut block);

            if i > 0 {
                encrypted_data.extend_from_slice(&block);
            } else {
                encrypted_data = block.to_vec();
            }
        }

        encrypted_data
    }
}

/// Decrypts the data using AES-128 decryption algorithm with zero-padding.
///
/// # Arguments
///
/// * `key` - A string representing the decryption key.
/// * `data` - A slice of u8 representing the data to be decrypted.
///
/// # Returns
///
/// A vector of u8 containing the decrypted data.
///
/// # Examples
///
/// ```
/// use stegano::utils::decrypt_data;
///
/// let key = "secret_key";
/// let encrypted_data: Vec<u8> = vec![1, 2, 3, 4, 5, 0, 0, 0, 0, 2, 3, 0, 0, 5, 0, 7];
/// let decrypted_data = decrypt_data(key, &encrypted_data);
/// assert_eq!(decrypted_data.len(), 16);
/// ```
pub fn decrypt_data(key: &str, data: &[u8]) -> Vec<u8> {
    let in_key: &[u8; 16] = &pad_with_zeros(key.as_bytes());
    let key = GenericArray::clone_from_slice(in_key);

    let mut decrypted_data: Vec<u8> = Vec::new();

    for (i, chunk) in data.chunks_exact(16).enumerate() {
        let in_payload: &[u8; 16] = &pad_with_zeros(chunk);
        let mut block = GenericArray::clone_from_slice(in_payload);

        let cipher = Aes128::new(&key);
        cipher.decrypt_block(&mut block);

        if i > 0 {
            decrypted_data.extend_from_slice(&block);
        } else {
            decrypted_data = block.to_vec();
        }
    }

    decrypted_data
}
