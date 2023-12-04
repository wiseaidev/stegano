use crate::jpeg::dct::DctStruct;
use crate::jpeg::writer::JpegWriter;

/// Represents the header for a Quantization Table (DQT) in a JPEG file.
///
/// The `DqtHeader` struct is used to store information about the quantization table in a JPEG file.
/// It contains a `dct` field, which is a `DctStruct` representing the quantization matrix.
/// This struct is typically used in conjunction with a JPEG writer to embed quantization table
/// information in the image file.
#[derive(Debug)]
pub struct DqtHeader {
    /// The quantization matrix represented as a `DctStruct`.
    pub dct: DctStruct,
}

impl DqtHeader {
    /// Creates a new `DqtHeader` instance with the specified quantization matrix.
    ///
    /// # Arguments
    ///
    /// * `dct` - A `DctStruct` representing the quantization matrix.
    ///
    /// # Returns
    ///
    /// A new `DqtHeader` instance with the provided quantization matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use stegano::jpeg::dct::DctStruct;
    /// use stegano::jpeg::dqt::DqtHeader;
    ///
    /// let bytes: Vec<u8> = vec![4; 128];
    /// let dct = DctStruct::new(&bytes).unwrap();
    ///
    /// let dqt_header = DqtHeader::new(dct);
    /// ```
    pub fn new(dct: DctStruct) -> Self {
        DqtHeader { dct }
    }

    /// Writes the quantization table header to a JPEG writer.
    ///
    /// This method takes a reference to a `JpegWriter` and writes the quantization table header,
    /// including the table identifier and the quantization values, to the underlying writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - A mutable reference to a `JpegWriter` trait object.
    ///
    /// # Examples
    ///
    /// ```
    /// use stegano::jpeg::dqt::DqtHeader;
    /// use stegano::jpeg::writer::JpegWriter;
    /// use stegano::jpeg::dct::DctStruct;
    /// use std::io::BufWriter;
    /// use std::fs::File;
    ///
    /// let bytes: Vec<u8> = vec![4; 128];
    /// let dct = DctStruct::new(&bytes).unwrap();
    ///
    /// let dqt_header = DqtHeader::new(dct);
    /// let output_file = File::create("temp.jpeg").unwrap();
    /// let mut writer = BufWriter::new(output_file);
    ///
    /// dqt_header.write(&mut writer);
    /// ```
    pub fn write(&self, writer: &mut dyn JpegWriter) {
        let mut dqt: Vec<u8> = vec![0xFF, 0xDB, 0x00, 0x84];
        for i in 0..2 {
            dqt.push(i as u8);
            let temp_array = &self.dct.quantum[i];
            dqt.extend(temp_array.iter().map(|&x| x as u8));
        }
        writer.write_array(&dqt);
    }
}
