use std::io::Write;

const JPEG_NATURAL_ORDER: [usize; 64] = [
    0, 1, 5, 6, 14, 15, 27, 28, 2, 4, 7, 13, 16, 26, 29, 42, 3, 8, 12, 17, 25, 30, 41, 43, 9, 11,
    18, 24, 31, 40, 44, 53, 10, 19, 23, 32, 39, 45, 52, 54, 20, 22, 33, 38, 46, 51, 55, 60, 21, 34,
    37, 47, 50, 56, 59, 61, 35, 36, 48, 49, 57, 58, 62, 63,
];

/// Represents a Huffman coding structure for encoding Discrete Cosine Transform coefficients.
///
/// This structure is used to perform Huffman encoding on quantized DCT coefficients obtained
/// from image processing operations. It includes tables for DC and AC components, as well
/// as methods for Huffman block encoding.
#[derive(Debug)]
#[allow(unused_variables, dead_code)]
pub struct Huffman {
    /// Number of bits in the buffer to be written.
    pub buffer_put_bits: i32,
    /// Buffer containing bits to be written.
    pub buffer_put_buffer: i32,
    /// Height of the image.
    pub image_height: i32,
    /// Width of the image.
    pub image_width: i32,
    /// DC matrix for luminance component.
    pub dc_matrix0: Vec<Vec<i32>>,
    /// AC matrix for luminance component.
    pub ac_matrix0: Vec<Vec<i32>>,
    /// DC matrix for chrominance component.
    pub dc_matrix1: Vec<Vec<i32>>,
    /// AC matrix for chrominance component.
    pub ac_matrix1: Vec<Vec<i32>>,
    /// 3D array storing DC matrices for luminance and chrominance components.
    pub dc_matrix: Vec<Vec<Vec<i32>>>,
    /// 3D array storing AC matrices for luminance and chrominance components.
    pub ac_matrix: Vec<Vec<Vec<i32>>>,
    /// Huffman code.
    pub code: i32,
    /// Number of DC tables.
    pub num_of_dc_tables: i32,
    /// Number of AC tables.
    pub num_of_ac_tables: i32,
    /// Bits for DC luminance component.
    pub bits_dc_luminance: Vec<i32>,
    /// Values for DC luminance component.
    pub val_dc_luminance: Vec<i32>,
    /// Bits for DC chrominance component.
    pub bits_dc_chrominance: Vec<i32>,
    /// Values for DC chrominance component.
    pub val_dc_chrominance: Vec<i32>,
    /// Bits for AC luminance component.
    pub bits_ac_luminance: Vec<i32>,
    /// Values for AC luminance component.
    pub val_ac_luminance: Vec<i32>,
    /// Bits for AC chrominance component.
    pub bits_ac_chrominance: Vec<i32>,
    /// Values for AC chrominance component.
    pub val_ac_chrominance: Vec<i32>,
    /// Vector storing bits for Huffman encoding.
    pub bits: Vec<Vec<i32>>,
    /// Vector storing values for Huffman encoding.
    pub val: Vec<Vec<i32>>,
}

impl Huffman {
    /// Creates a new `Huffman` struct with predefined tables and settings.
    ///
    /// This method is used to initialize a `Huffman` struct with default values for Huffman
    /// encoding of Discrete Cosine Transform (DCT) coefficients. The Huffman tables for DC and AC
    /// components are pre-defined based on the JPEG standard.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the image.
    /// * `height` - Height of the image.
    ///
    /// # Returns
    ///
    /// A `Huffman` struct initialized with default values for Huffman encoding.
    ///
    /// # Examples
    ///
    /// ```
    /// use stegano::jpeg::huff::Huffman;
    ///
    /// // Assuming width and height are the dimensions of the image
    /// let width = 640;
    /// let height = 480;
    /// let huffman_encoder = Huffman::new(width, height);
    ///
    /// // Verify the initialized Huffman struct
    /// println!("Huffman Encoder initialized: {:?}", huffman_encoder);
    ///
    /// // Assertions for the initialized fields of the Huffman struct
    /// assert_eq!(huffman_encoder.buffer_put_bits, 0);
    /// assert_eq!(huffman_encoder.buffer_put_buffer, 0);
    /// assert_eq!(huffman_encoder.image_height, 480);
    /// assert_eq!(huffman_encoder.image_width, 640);
    /// assert_eq!(huffman_encoder.dc_matrix0, vec![vec![0; 2]; 12]);
    /// assert_eq!(huffman_encoder.ac_matrix0, vec![vec![0; 2]; 255]);
    /// assert_eq!(huffman_encoder.dc_matrix1, vec![vec![0; 2]; 12]);
    /// assert_eq!(huffman_encoder.ac_matrix1, vec![vec![0; 2]; 255]);
    /// assert_eq!(huffman_encoder.dc_matrix, vec![vec![vec![0; 2]; 12]; 2]);
    /// assert_eq!(huffman_encoder.ac_matrix, vec![vec![vec![0; 2]; 255]; 2]);
    /// assert_eq!(huffman_encoder.code, 0);
    /// assert_eq!(huffman_encoder.num_of_dc_tables, 0);
    /// assert_eq!(huffman_encoder.num_of_ac_tables, 0);
    /// assert_eq!(huffman_encoder.bits_dc_luminance, vec![0, 1, 5, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0]);
    /// assert_eq!(huffman_encoder.val_dc_luminance, (0..12).collect::<Vec<_>>());
    /// assert_eq!(huffman_encoder.bits_dc_chrominance, vec![0, 1, 3, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0]);
    /// assert_eq!(huffman_encoder.val_dc_chrominance, (0..12).collect::<Vec<_>>());
    /// assert_eq!(huffman_encoder.bits_ac_luminance, vec![0, 2, 1, 3, 3, 2, 4, 3, 5, 5, 4, 4, 0, 0, 0, 0, 1]);
    /// assert_eq!(huffman_encoder.val_ac_luminance.len(), 162);
    /// assert_eq!(huffman_encoder.bits_ac_chrominance, vec![0, 2, 1, 2, 4, 4, 3, 4, 7, 5, 4, 4, 0, 0, 0, 0, 1]);
    /// assert_eq!(huffman_encoder.val_ac_chrominance.len(), 162);
    /// assert_eq!(huffman_encoder.bits.len(), 4);
    /// assert_eq!(huffman_encoder.val.len(), 4);
    /// ```
    ///
    /// The resulting `Huffman` struct contains pre-defined Huffman tables for luminance and chrominance
    /// components, along with other necessary settings for Huffman encoding of DCT coefficients.
    pub fn new(width: i32, height: i32) -> Huffman {
        let mut bits_dc_luminance = vec![0; 17];
        bits_dc_luminance[1] = 1;
        bits_dc_luminance[2] = 5;
        bits_dc_luminance[3] = 1;
        bits_dc_luminance[4] = 1;
        bits_dc_luminance[5] = 1;
        bits_dc_luminance[6] = 1;
        bits_dc_luminance[7] = 1;
        bits_dc_luminance[8] = 1;
        bits_dc_luminance[9] = 1;
        bits_dc_luminance[15] = 1;

        let mut val_dc_luminance = vec![0i32; 12];
        for (i, el) in val_dc_luminance.iter_mut().enumerate() {
            *el = i as i32;
        }

        let mut bits_ac_luminance = vec![0; 17];
        bits_ac_luminance[1] = 2;
        bits_ac_luminance[2] = 1;
        bits_ac_luminance[3] = 3;
        bits_ac_luminance[4] = 3;
        bits_ac_luminance[5] = 2;
        bits_ac_luminance[6] = 4;
        bits_ac_luminance[7] = 3;
        bits_ac_luminance[8] = 5;
        bits_ac_luminance[9] = 5;
        bits_ac_luminance[10] = 4;
        bits_ac_luminance[11] = 4;
        bits_ac_luminance[16] = 1;

        let val_ac_luminance = vec![
            1, 2, 3, 0, 4, 17, 5, 18, 33, 49, 6, 65, 81, 7, 97, 113, 19, 34, 50, 129, 8, 20, 66,
            145, 161, 177, 193, 9, 35, 51, 82, 240, 21, 98, 114, 209, 10, 22, 36, 52, 225, 37, 241,
            23, 24, 25, 26, 38, 39, 40, 41, 42, 53, 54, 55, 56, 57, 58, 67, 68, 69, 70, 71, 72, 73,
            74, 83, 84, 85, 86, 87, 88, 89, 90, 99, 100, 101, 102, 103, 104, 105, 106, 115, 116,
            117, 118, 119, 120, 121, 122, 130, 131, 132, 133, 134, 135, 136, 137, 138, 146, 147,
            148, 149, 150, 151, 152, 153, 154, 162, 163, 164, 165, 166, 167, 168, 169, 170, 178,
            179, 180, 181, 182, 183, 184, 185, 186, 194, 195, 196, 197, 198, 199, 200, 201, 202,
            210, 211, 212, 213, 214, 215, 216, 217, 218, 226, 227, 228, 229, 230, 231, 232, 233,
            234, 242, 243, 244, 245, 246, 247, 248, 249, 250,
        ];

        let mut bits_dc_chrominance = vec![0; 17];
        bits_dc_chrominance[1] = 1;
        bits_dc_chrominance[2] = 3;
        bits_dc_chrominance[3] = 1;
        bits_dc_chrominance[4] = 1;
        bits_dc_chrominance[5] = 1;
        bits_dc_chrominance[6] = 1;
        bits_dc_chrominance[7] = 1;
        bits_dc_chrominance[8] = 1;
        bits_dc_chrominance[9] = 1;
        bits_dc_chrominance[15] = 1;

        let mut val_dc_chrominance = vec![0i32; 12];
        for (i, el) in val_dc_chrominance.iter_mut().enumerate() {
            *el = i as i32;
        }

        let mut bits_ac_chrominance = vec![0; 17];
        bits_ac_chrominance[1] = 2;
        bits_ac_chrominance[2] = 1;
        bits_ac_chrominance[3] = 2;
        bits_ac_chrominance[4] = 4;
        bits_ac_chrominance[5] = 4;
        bits_ac_chrominance[6] = 3;
        bits_ac_chrominance[7] = 4;
        bits_ac_chrominance[8] = 7;
        bits_ac_chrominance[9] = 5;
        bits_ac_chrominance[10] = 4;
        bits_ac_chrominance[11] = 4;
        bits_ac_chrominance[16] = 1;

        let val_ac_chrominance = vec![
            0, 1, 2, 3, 17, 4, 5, 18, 33, 49, 6, 65, 81, 7, 97, 113, 19, 34, 50, 129, 8, 20, 66,
            145, 161, 177, 193, 9, 35, 51, 82, 240, 21, 98, 114, 209, 10, 22, 36, 52, 225, 37, 241,
            23, 24, 25, 26, 38, 39, 40, 41, 42, 53, 54, 55, 56, 57, 58, 67, 68, 69, 70, 71, 72, 73,
            74, 83, 84, 85, 86, 87, 88, 89, 90, 99, 100, 101, 102, 103, 104, 105, 106, 115, 116,
            117, 118, 119, 120, 121, 122, 130, 131, 132, 133, 134, 135, 136, 137, 138, 146, 147,
            148, 149, 150, 151, 152, 153, 154, 162, 163, 164, 165, 166, 167, 168, 169, 170, 178,
            179, 180, 181, 182, 183, 184, 185, 186, 194, 195, 196, 197, 198, 199, 200, 201, 202,
            210, 211, 212, 213, 214, 215, 216, 217, 218, 226, 227, 228, 229, 230, 231, 232, 233,
            234, 242, 243, 244, 245, 246, 247, 248, 249, 250,
        ];

        let bits = vec![
            bits_dc_luminance.clone(),
            bits_ac_luminance.clone(),
            bits_dc_chrominance.clone(),
            bits_ac_chrominance.clone(),
        ];

        let val = vec![
            val_dc_luminance.clone(),
            val_ac_luminance.clone(),
            val_dc_chrominance.clone(),
            val_ac_chrominance.clone(),
        ];

        Huffman {
            buffer_put_bits: 0,
            buffer_put_buffer: 0,
            image_height: height,
            image_width: width,
            dc_matrix0: vec![vec![0; 2]; 12],
            ac_matrix0: vec![vec![0; 2]; 255],
            dc_matrix1: vec![vec![0; 2]; 12],
            ac_matrix1: vec![vec![0; 2]; 255],
            dc_matrix: vec![vec![vec![0; 2]; 12]; 2],
            ac_matrix: vec![vec![vec![0; 2]; 255]; 2],
            code: 0,
            num_of_dc_tables: 0,
            num_of_ac_tables: 0,
            bits_dc_luminance,
            val_dc_luminance,
            bits_dc_chrominance,
            val_dc_chrominance,
            bits_ac_luminance,
            val_ac_luminance,
            bits_ac_chrominance,
            val_ac_chrominance,
            bits,
            val,
        }
    }

    /// Huffman block encoder for encoding DC and AC coefficients.
    ///
    /// This method encodes a block of Discrete Cosine Transform (DCT) coefficients using Huffman
    /// encoding. It processes both the DC and AC components of the block and writes the encoded
    /// data to the specified output stream.
    ///
    /// # Arguments
    ///
    /// * `out_stream` - A mutable reference to a trait object implementing the `Write` trait. The
    ///   encoded data will be written to this stream.
    /// * `zigzag` - A reference to a slice containing the zigzag-ordered DCT coefficients for the
    ///   block.
    /// * `prec` - The predictor value for DC encoding.
    /// * `dc_code` - The DC Huffman table index.
    /// * `ac_code` - The AC Huffman table index.
    ///
    /// # Panics
    ///
    /// This method panics if the provided Huffman table indices are out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stegano::jpeg::huff::Huffman; // Replace with the actual crate name
    /// use std::io::Cursor;
    ///
    /// // Assuming huffman_encoder is a properly initialized Huffman struct
    /// let mut huffman_encoder = Huffman::new(640, 480);
    ///
    /// // Sample zigzag-ordered DCT coefficients and other required parameters
    /// let zigzag_coefficients: [i32; 64] = [
    ///     20, -2, 1, 0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0, 0, 0, 0,
    /// ];

    /// let predictor = 0;
    /// let dc_table_index = 0;
    /// let ac_table_index = 1;
    ///
    /// // Create a buffer to hold the encoded data
    /// let mut encoded_buffer = Vec::new();
    ///
    /// // Encode the block
    /// huffman_encoder.huffman_block_encoder(&mut encoded_buffer, &zigzag_coefficients, predictor, dc_table_index, ac_table_index);
    ///
    /// // Verify the encoded data
    /// assert!(!encoded_buffer.is_empty(), "Encoded buffer should not be empty");
    /// ```
    pub fn huffman_block_encoder(
        &mut self,
        out_stream: &mut dyn Write,
        zigzag: &[i32; 64],
        prec: i32,
        dc_code: i32,
        ac_code: i32,
    ) {
        let mut temp;
        let mut temp2;
        let mut nbits;

        self.num_of_dc_tables = 2;
        self.num_of_ac_tables = 2;

        // The DC portion
        temp = zigzag[0] - prec;
        temp2 = temp;
        if temp < 0 {
            temp = -temp;
            temp2 -= 1;
        }
        nbits = 0;
        while temp != 0 {
            nbits += 1;
            temp >>= 1;
        }

        self.buffer_it(
            out_stream,
            self.dc_matrix[dc_code as usize][nbits][0],
            self.dc_matrix[dc_code as usize][nbits][1],
        );
        // The arguments in buffer_it are code and size.
        if nbits != 0 {
            self.buffer_it(out_stream, temp2, nbits.try_into().unwrap());
        }

        // The AC portion
        let mut r = 0;
        let mut _i = 0;
        for k in 1..64 {
            let mut temp = zigzag[JPEG_NATURAL_ORDER[k]];
            if temp == 0 {
                r += 1;
            } else {
                while r > 15 {
                    self.buffer_it(
                        out_stream,
                        self.ac_matrix[ac_code as usize][0xF0][0],
                        self.ac_matrix[ac_code as usize][0xF0][1],
                    );
                    r -= 16;
                }
                temp2 = temp;
                if temp < 0 {
                    temp = -temp;
                    temp2 -= 1;
                }
                nbits = 1;
                while temp != 0 {
                    nbits += 1;
                    temp >>= 1;
                }
                _i = (r << 4) + nbits;
                self.buffer_it(
                    out_stream,
                    self.ac_matrix[ac_code as usize][_i][0],
                    self.ac_matrix[ac_code as usize][_i][1],
                );
                self.buffer_it(out_stream, temp2, nbits.try_into().unwrap());
                r = 0;
            }
        }

        if r > 0 {
            self.buffer_it(
                out_stream,
                self.ac_matrix[ac_code as usize][0][0],
                self.ac_matrix[ac_code as usize][0][1],
            );
        }
    }

    /// Writes a Huffman-encoded code to the output stream with specified size.
    ///
    /// This function is responsible for buffering Huffman-encoded codes before writing them to the output stream.
    ///
    /// # Arguments
    ///
    /// * `out_stream` - A mutable reference to a type implementing the `Write` trait, where the encoded data will be written.
    /// * `code` - The Huffman-encoded code to be written.
    /// * `size` - The size (number of bits) of the Huffman-encoded code.
    ///
    /// # Panics
    ///
    /// Panics if the output stream encounters an error during writing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use stegano::jpeg::huff::Huffman;
    ///
    /// let mut huffman_encoder = Huffman::new(8, 8);
    /// let mut output_buffer = Cursor::new(Vec::new());
    ///
    /// // Example Huffman-encoded code and size
    /// let code = 0b110110; // 6 bits
    /// let size = 6;
    ///
    /// // Assuming code and size are properly initialized
    /// huffman_encoder.buffer_it(&mut output_buffer, code, size);
    ///
    /// assert_eq!(huffman_encoder.bits.len(), 4);
    /// assert_eq!(huffman_encoder.buffer_put_bits, 6);
    /// assert_eq!(huffman_encoder.buffer_put_buffer, 14155776);
    /// ```
    pub fn buffer_it(&mut self, out_stream: &mut dyn Write, code: i32, size: i32) {
        let mut put_buffer = code;
        let mut put_bits = self.buffer_put_bits;

        put_buffer &= (1 << size) - 1;
        put_bits += size;
        put_buffer <<= 24 - put_bits;
        put_buffer |= self.buffer_put_buffer;

        while put_bits >= 8 {
            let c = ((put_buffer >> 16) & 0xFF) as u8;
            out_stream.write_all(&[c]).unwrap();
            if c == 0xFF {
                out_stream.write_all(&[0]).unwrap();
            }
            put_buffer <<= 8;
            put_bits -= 8;
        }
        self.buffer_put_buffer = put_buffer;
        self.buffer_put_bits = put_bits;
    }

    /// Flushes the internal buffer to the output stream, writing any remaining bits.
    ///
    /// This function is typically called at the end of encoding to ensure that all bits
    /// are written to the output stream.
    ///
    /// # Arguments
    ///
    /// * `out_stream` - A mutable reference to a type implementing the `Write` trait,
    ///                  where the encoded data will be written.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use stegano::jpeg::huff::Huffman;
    ///
    /// // Create a new Huffman instance
    /// let mut huffman_encoder = Huffman::new(8, 8);
    ///
    /// // Create a buffer for the output stream
    /// let mut output_buffer = Cursor::new(Vec::new());
    ///
    /// // Example Huffman-encoded code and size
    /// let code = 0b110110; // 6 bits
    /// let size = 6;
    ///
    /// // Call the buffer_it function
    /// huffman_encoder.buffer_it(&mut output_buffer, code, size);
    ///
    /// // Flush the buffer
    /// huffman_encoder.flush_buffer(&mut output_buffer);
    ///
    /// // Check the buffered output length after flushing
    /// let buffered_length = output_buffer.get_ref().len();
    ///
    /// // Assertions for the fields of the Huffman struct after flushing
    /// assert_eq!(huffman_encoder.buffer_put_bits, 0);
    /// assert_eq!(huffman_encoder.buffer_put_buffer, 0);
    ///
    /// // Assertions for the buffered output length after flushing
    /// assert_eq!(buffered_length, 1);
    /// ```
    pub fn flush_buffer(&mut self, out_stream: &mut dyn Write) {
        let mut put_buffer = self.buffer_put_buffer;
        let mut put_bits = self.buffer_put_bits;

        while put_bits >= 8 {
            let c = ((put_buffer >> 16) & 0xFF) as u8;
            out_stream.write_all(&[c]).unwrap();
            if c == 0xFF {
                out_stream.write_all(&[0]).unwrap();
            }
            put_buffer <<= 8;
            put_bits -= 8;
        }

        if put_bits > 0 {
            let c = ((put_buffer >> 16) & 0xFF) as u8;
            out_stream.write_all(&[c]).unwrap();
        }

        self.buffer_put_buffer = 0;
        self.buffer_put_bits = 0;
    }
}
