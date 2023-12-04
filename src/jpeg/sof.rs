use crate::jpeg::obj::JpegObj;
use crate::jpeg::writer::JpegWriter;

/// Represents the header for the Start of Frame (SOF) in a JPEG file.
///
/// The `SofHeader` struct is used to store information about the image dimensions and components
/// in the Start of Frame header of a JPEG file. It contains a `jpeg_obj` field, which is a
/// `JpegObj` representing various properties such as precision, image dimensions, and component details.
/// This struct is typically used in conjunction with a JPEG writer to embed Start of Frame header
/// information in the image file.
#[derive(Debug)]
pub struct SofHeader {
    /// The `JpegObj` representing various properties of the image.
    pub jpeg_obj: JpegObj,
}

impl SofHeader {
    /// Creates a new `SofHeader` instance with the specified `JpegObj`.
    ///
    /// # Arguments
    ///
    /// * `jpeg_obj` - A `JpegObj` representing various properties of the image.
    ///
    /// # Returns
    ///
    /// A new `SofHeader` instance with the provided `JpegObj`.
    ///
    /// # Examples
    ///
    /// ```
    /// use stegano::jpeg::sof::SofHeader;
    /// use stegano::jpeg::obj::JpegObj;
    ///
    /// let jpeg_obj = JpegObj::default();
    /// let sof_header = SofHeader::new(jpeg_obj);
    /// ```
    pub fn new(jpeg_obj: JpegObj) -> Self {
        SofHeader { jpeg_obj }
    }

    /// Writes the Start of Frame header to a JPEG writer.
    ///
    /// This method takes a reference to a `JpegWriter` and writes the Start of Frame header,
    /// including the precision, image dimensions, and component details, to the underlying writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - A mutable reference to a `JpegWriter` trait object.
    ///
    /// # Examples
    ///
    /// ```
    /// use stegano::jpeg::sof::SofHeader;
    /// use stegano::jpeg::obj::JpegObj;
    /// use std::io::BufWriter;
    /// use std::fs::File;
    ///
    /// let output_file = File::create("temp.jpeg").unwrap();
    /// let mut writer = BufWriter::new(output_file);
    ///
    /// let jpeg_obj = JpegObj::default();
    /// let sof_header = SofHeader::new(jpeg_obj);
    /// sof_header.write(&mut writer);
    /// ```
    pub fn write(&self, writer: &mut dyn JpegWriter) {
        let mut sof: Vec<u8> = vec![0xFF, 0xC0, 0x00, 17];
        sof.push(self.jpeg_obj.precision);
        sof.push((self.jpeg_obj.image_height >> 8) as u8);
        sof.push(self.jpeg_obj.image_height as u8);
        sof.push((self.jpeg_obj.image_width >> 8) as u8);
        sof.push(self.jpeg_obj.image_width as u8);
        sof.push(self.jpeg_obj.number_of_components);
        for i in 0..self.jpeg_obj.number_of_components as usize {
            sof.push(self.jpeg_obj.comp_id[i]);
            sof.push((self.jpeg_obj.hsamp_factor[i] << 4) + self.jpeg_obj.vsamp_factor[i]);
            sof.push(self.jpeg_obj.qtable_number[i]);
        }
        writer.write_array(&sof);
    }
}
