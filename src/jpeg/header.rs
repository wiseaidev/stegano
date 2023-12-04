use crate::jpeg::writer::JpegWriter;

/// Struct representing the header of a JPEG File Interchange Format (JFIF) file.
///
/// This struct contains information about the JFIF version.
#[derive(Debug)]
pub struct JfifHeader {
    /// JFIF version information.
    pub version: u16,
}

impl JfifHeader {
    /// Creates a new `JfifHeader` by parsing a byte slice.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to a byte slice containing the JFIF header data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `JfifHeader` if successful, or an error message
    /// if the byte slice is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use stegano::jpeg::header::JfifHeader;
    ///
    /// // Assuming data is a valid byte slice containing JFIF header data
    /// let data: Vec<u8> = vec![
    ///     0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
    ///     0x00, 0x01, 0x00, 0x00,
    /// ];
    ///
    /// let jfif_header_result = JfifHeader::new(&data);
    ///
    /// match jfif_header_result {
    ///     Ok(jfif_header) => {
    ///         assert_eq!(jfif_header.version, 1);
    ///
    ///         println!("JfifHeader created successfully: {:?}", jfif_header);
    ///     }
    ///     Err(err) => {
    ///         eprintln!("Error creating JfifHeader: {}", err);
    ///     }
    /// }
    /// ```
    pub fn new(data: &[u8]) -> Result<Self, &'static str> {
        // Check if the byte slice has the expected length
        if data.len() != 18 {
            eprintln!("Warning: Invalid byte slice length for JFIF header. Continuing...");
            return Err("Invalid byte slice length for JFIF header");
        }

        // Validate the JFIF marker and other fields
        let expected_marker: [u8; 2] = [0x4A, 0x46]; // ASCII codes for "JF"
        if data[0..2] != expected_marker {
            eprintln!("Warning: Invalid JFIF marker. Continuing...");
            return Err("Invalid JFIF marker");
        }

        // Extract the version field from the byte slice
        let version = u16::from_be_bytes(data[2..4].try_into().unwrap());

        // If all checks pass, create and return the JfifHeader
        Ok(JfifHeader { version })
    }

    /// Writes the JFIF header to the provided JPEG writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - A mutable reference to a `JpegWriter` trait object.
    ///
    /// # Examples
    ///
    /// ```
    /// use stegano::jpeg::header::JfifHeader;
    /// use std::io::BufWriter;
    /// use std::fs::File;
    ///
    /// let output_file = File::create("temp.jpeg").unwrap();
    ///
    /// let mut writer = BufWriter::new(output_file);
    /// let jfif_header = JfifHeader { version: 1 };
    /// jfif_header.write(&mut writer);
    /// ```
    pub fn write(&self, writer: &mut dyn JpegWriter) {
        let jfif: [u8; 18] = [
            0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x01, 0x00, 0x00,
        ];
        writer.write_array(&jfif);
    }
}
