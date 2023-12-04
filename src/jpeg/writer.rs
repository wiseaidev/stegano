use std::io::Write;

/// Trait for writing JPEG markers and arrays to a writer.
///
/// This trait defines methods for writing JPEG markers and arrays to a byte stream or any type
/// implementing the `std::io::Write` trait.
pub trait JpegWriter {
    /// Writes a JPEG marker to the writer.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to a byte slice containing the marker data.
    fn write_marker(&mut self, data: &[u8]);

    /// Writes a JPEG array to the writer.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to a byte slice containing the array data.
    fn write_array(&mut self, data: &[u8]);
}

/// Implements the `JpegWriter` trait for any type that implements the `std::io::Write` trait.
///
/// This implementation allows writing JPEG markers and arrays to any type that supports writing
/// bytes, such as file handles or in-memory buffers.
impl<W: Write> JpegWriter for W {
    /// Writes a JPEG marker to the writer.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to a byte slice containing the marker data.
    fn write_marker(&mut self, data: &[u8]) {
        self.write_all(data).expect("IO Error");
    }

    /// Writes a JPEG array to the writer.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to a byte slice containing the array data.
    fn write_array(&mut self, data: &[u8]) {
        if data.len() >= 4 {
            let length = ((data[2] as usize) << 8) + (data[3] as usize) + 2;
            self.write_all(&data[..length.min(data.len())])
                .expect("IO Error");
        } else {
            // Handle the case where the slice is too short
            eprintln!("Error: Data slice is too short in write_array.");
        }
    }
}

/// Implements the `JpegWriter` trait for a dynamic trait object of type `std::io::Write`.
///
/// This implementation allows writing JPEG markers and arrays to any trait object that implements
/// the `std::io::Write` trait.
impl JpegWriter for dyn std::io::Write {
    /// Writes a JPEG marker to the writer.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to a byte slice containing the marker data.
    fn write_marker(&mut self, data: &[u8]) {
        self.write_all(data).expect("IO Error");
    }

    /// Writes a JPEG array to the writer.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to a byte slice containing the array data.
    fn write_array(&mut self, data: &[u8]) {
        let length = ((data[2] as usize) << 8) + (data[3] as usize) + 2;
        self.write_all(&data[..length]).expect("IO Error");
    }
}
