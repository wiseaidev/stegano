/// Represents a structure for storing Discrete Cosine Transform coefficients.
///
/// This structure is specifically designed to store luminance and chrominance coefficients
/// obtained from image processing operations.
#[derive(Debug)]
pub struct DctStruct {
    /// 2D array storing the DCT coefficients. The outer array has a length of 2, representing
    /// luminance and chrominance components, and the inner array has a length of 64, representing
    /// the coefficients for each component.
    pub quantum: [[u16; 64]; 2],
}

/// Implements the default constructor for `DctStruct`.
///
/// The default constructor initializes a new `DctStruct` with all coefficients set to zero.
impl Default for DctStruct {
    fn default() -> Self {
        DctStruct {
            quantum: [[0; 64], [0; 64]],
        }
    }
}

/// Implements additional methods for the `DctStruct` type.
impl DctStruct {
    /// Creates a new `DctStruct` by parsing a byte slice.
    ///
    /// This method is particularly useful for deserializing DCT coefficients from a byte stream,
    /// such as when reading from an image file.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A reference to a byte slice containing serialized DCT coefficients.
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `DctStruct` if successful, or an error message
    /// if the byte slice is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use stegano::jpeg::dct::DctStruct;
    ///
    /// // Assuming bytes is a valid byte slice containing DCT coefficients
    /// let bytes: Vec<u8> = vec![4; 128];
    /// let dct_result = DctStruct::new(&bytes);
    ///
    /// match dct_result {
    ///    Ok(dct) => {
    ///        // Verify that the coefficients are not all zeros
    ///        assert!(dct.quantum.iter().any(|row| row.iter().any(|&coeff| coeff != 0)));
    ///
    ///        // Verify that the structure has the expected dimensions:
    ///        assert_eq!(dct.quantum.len(), 2);
    ///        assert_eq!(dct.quantum[0].len(), 64);
    ///        assert_eq!(dct.quantum[1].len(), 64);
    ///
    ///        println!("DctStruct created successfully: {:?}", dct);
    ///    }
    ///     Err(err) => {
    ///         eprintln!("Error creating DctStruct: {}", err);
    ///     }
    /// }
    /// ```
    pub fn new(bytes: &[u8]) -> Result<Self, &'static str> {
        // Check if the byte slice has the expected length
        if bytes.len() != 128 {
            return Err("Invalid byte slice length for DctStruct");
        }

        // Parse the byte slice into the quantum field
        let mut quantum = [[0; 64]; 2];

        for (i, row) in quantum.iter_mut().enumerate() {
            for (j, coeff) in row.iter_mut().enumerate() {
                let index = i * 64 + j;
                if index + 1 < bytes.len() {
                    *coeff = u16::from_be_bytes(bytes[index..(index + 2)].try_into().unwrap());
                } else {
                    eprintln!("Invalid byte slice for DctStruct");
                    break;
                }
            }
        }

        // Return a Result with the created DctStruct on success
        Ok(DctStruct { quantum })
    }
}
