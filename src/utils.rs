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
