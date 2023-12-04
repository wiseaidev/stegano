use crate::jpeg::writer::JpegWriter;

/// Represents the header for a comment in a JPEG file.
///
/// The `CommentHeader` struct is used to store comment information in a JPEG file. It contains
/// a `comment` field, which holds the actual text of the comment. This struct is typically
/// used in conjunction with a JPEG writer to embed comments in the image file.
#[derive(Debug)]
pub struct CommentHeader {
    /// The actual text of the comment.
    pub comment: String,
}

impl CommentHeader {
    /// Creates a new `CommentHeader` instance with the specified comment.
    ///
    /// # Arguments
    ///
    /// * `comment` - A string slice representing the comment to be stored.
    ///
    /// # Returns
    ///
    /// A new `CommentHeader` instance with the provided comment.
    ///
    /// # Examples
    ///
    /// ```
    /// use stegano::jpeg::comment::CommentHeader;
    ///
    /// let comment = CommentHeader::new("This is a sample comment.");
    /// ```
    pub fn new(comment: &str) -> Self {
        CommentHeader {
            comment: comment.to_string(),
        }
    }

    /// Writes the comment header to a JPEG writer.
    ///
    /// This method takes a reference to a `JpegWriter` and writes the comment header, including
    /// the length of the comment, to the underlying writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - A mutable reference to a `JpegWriter` trait object.
    ///
    /// # Examples
    ///
    /// ```
    /// use stegano::jpeg::comment::CommentHeader;
    /// use std::io::BufWriter;
    /// use std::fs::File;
    ///
    /// let output_file = File::create("temp.jpeg").unwrap();
    /// let mut writer = BufWriter::new(output_file);
    ///
    /// let comment = CommentHeader::new("This is a sample comment.");
    /// comment.write(&mut writer);
    /// ```
    pub fn write(&self, writer: &mut dyn JpegWriter) {
        let length = self.comment.len();
        let com: Vec<u8> = vec![
            0xFF,
            0xFE,
            ((length >> 8) & 0xFF) as u8,
            (length & 0xFF) as u8,
        ];
        let comment_bytes: Vec<u8> = self.comment.bytes().collect();
        let com = [&com[..], &comment_bytes[..]].concat();
        writer.write_array(&com);
    }
}
