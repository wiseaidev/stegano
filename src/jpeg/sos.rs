use crate::jpeg::obj::JpegObj;
use crate::jpeg::writer::JpegWriter;

/// Represents the Start of Scan (SOS) header in a JPEG file.
///
/// The `SosHeader` struct is used to store information related to the start of a scan in a JPEG image.
/// It includes a `jpeg_obj` field, which is an instance of the `JpegObj` struct representing the JPEG image.
/// This struct is typically used in conjunction with a JPEG writer to encode the SOS header in the image file.
#[derive(Debug, Clone)]
pub struct SosHeader {
    /// An instance of the `JpegObj` struct representing the JPEG image.
    pub jpeg_obj: JpegObj,
}

impl SosHeader {
    /// Creates a new `SosHeader` instance with the specified `JpegObj`.
    ///
    /// # Arguments
    ///
    /// * `jpeg_obj` - An instance of the `JpegObj` struct representing the JPEG image.
    ///
    /// # Returns
    ///
    /// A new `SosHeader` instance with the provided `JpegObj`.
    ///
    /// # Examples
    ///
    /// ```
    /// use stegano::jpeg::sos::SosHeader;
    /// use stegano::jpeg::obj::JpegObj;
    ///
    /// let jpeg_obj = JpegObj::default();
    /// let sof_header = SosHeader::new(jpeg_obj);
    /// ```
    pub fn new(jpeg_obj: JpegObj) -> Self {
        SosHeader { jpeg_obj }
    }

    /// Writes the SOS header to a JPEG writer.
    ///
    /// This method takes a reference to a `JpegWriter` and writes the SOS header, including
    /// information about the number of components, component IDs, DCT/AC table numbers, and
    /// other relevant information to the underlying writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - A mutable reference to a `JpegWriter` trait object.
    ///
    /// # Examples
    ///
    /// ```
    /// use stegano::jpeg::sos::SosHeader;
    /// use stegano::jpeg::obj::JpegObj;
    /// use std::io::BufWriter;
    /// use std::fs::File;
    ///
    /// let output_file = File::create("temp.jpeg").unwrap();
    /// let mut writer = BufWriter::new(output_file);
    ///
    /// let jpeg_obj = JpegObj::default();
    /// let sof_header = SosHeader::new(jpeg_obj);
    /// sof_header.write(&mut writer);
    /// ```
    pub fn write(&self, writer: &mut dyn JpegWriter) {
        let mut sos: Vec<u8> = vec![0xFF, 0xDA, 0x00, 12];
        sos.push(self.jpeg_obj.number_of_components);

        for i in 0..self
            .jpeg_obj
            .number_of_components
            .min(self.jpeg_obj.comp_id.len() as u8) as usize
        {
            sos.push(self.jpeg_obj.comp_id[i]);
            sos.push((self.jpeg_obj.dctable_number[i] << 4) + self.jpeg_obj.actable_number[i]);
        }

        sos.push(self.jpeg_obj.ss);
        sos.push(self.jpeg_obj.se);
        sos.push((self.jpeg_obj.ah << 4) + self.jpeg_obj.al);
        writer.write_array(&sos);
    }
}
