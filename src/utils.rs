use std::mem;

/// Performs XOR encoding or decoding on the provided byte slice using the specified key.
///
/// # Arguments
///
/// * `input` - The input byte slice to be encoded or decoded.
/// * `key` - The key used for XOR encoding or decoding. It is repeated cyclically if shorter than the input.
///
/// # Returns
///
/// A `Vec<u8>` containing the result of the XOR operation.
///
/// # Examples
///
/// ```
/// use stegano::utils::xor_encode_decode;
///
/// let input = b"Hello, World!";
/// let key = "secret_key";
/// let encoded = xor_encode_decode(input, key);
/// let decoded = xor_encode_decode(&encoded, key);
/// assert_eq!(input, decoded.as_slice());
/// ```
pub fn xor_encode_decode(input: &[u8], key: &str) -> Vec<u8> {
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
