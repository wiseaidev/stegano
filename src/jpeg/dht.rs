use crate::jpeg::huff::Huffman;
use crate::jpeg::writer::JpegWriter;

/// Represents the header for a DHT (Define Huffman Table) segment in a JPEG file.
///
/// The `DhtHeader` struct is used to store information related to Huffman coding tables in a JPEG file.
/// It contains a `huf` field, which is an instance of `Huffman` representing the Huffman coding information.
/// This struct is typically used in conjunction with a JPEG writer to embed Huffman coding tables in the image file.
#[derive(Debug)]
pub struct DhtHeader {
    /// The Huffman coding information stored in a separate struct.
    pub huf: Huffman,
}

impl DhtHeader {
    /// Creates a new `DhtHeader` instance with the specified Huffman coding information.
    ///
    /// # Arguments
    ///
    /// * `huf` - An instance of `Huffman` representing the Huffman coding information.
    ///
    /// # Returns
    ///
    /// A new `DhtHeader` instance with the provided Huffman coding information.
    ///
    /// # Examples
    ///
    /// ```
    /// use stegano::jpeg::dht::DhtHeader;
    /// use stegano::jpeg::huff::Huffman;
    ///
    /// // Assuming width and height are the dimensions of the image
    /// let width = 640;
    /// let height = 480;
    /// let huffman_encoder = Huffman::new(width, height);
    /// let dht_header = DhtHeader::new(huffman_encoder);
    /// ```
    pub fn new(huf: Huffman) -> Self {
        DhtHeader { huf }
    }

    /// Writes the DHT segment to a JPEG writer.
    ///
    /// This method takes a reference to a `JpegWriter` and writes the DHT segment to the underlying writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - A mutable reference to a `JpegWriter` trait object.
    ///
    /// # Examples
    ///
    /// ```
    /// use stegano::jpeg::dht::DhtHeader;
    /// use stegano::jpeg::huff::Huffman;
    /// use stegano::jpeg::writer::JpegWriter;
    /// use std::io::BufWriter;
    /// use std::fs::File;
    ///
    /// let width = 640;
    /// let height = 480;
    /// let huffman_encoder = Huffman::new(width, height);
    /// let dht_header = DhtHeader::new(huffman_encoder);
    /// let output_file = File::create("temp.jpeg").unwrap();
    /// let mut writer = BufWriter::new(output_file);
    /// dht_header.write(&mut writer);
    /// ```
    pub fn write(&self, writer: &mut dyn JpegWriter) {
        let mut dht: Vec<u8> = vec![0xFF, 0xC4];
        let index = 4;
        let mut old_index = 4;
        for i in 0..4 {
            let bytes = self.huf.bits[i][0];
            dht.push(bytes as u8);
            for j in 1..17 {
                let temp = self.huf.bits[i][j];
                dht.push(temp as u8);
            }
            let dht2: Vec<u8> = self.huf.val[i].iter().map(|&x| x as u8).collect();
            dht.extend_from_slice(&dht2);
            let mut dht3: Vec<u8> = vec![0xFF, 0xC4];
            let end_index = old_index + 17 + bytes as usize;
            dht3.extend_from_slice(&dht[old_index..end_index.min(index)]); // Ensure the range is within bounds
            dht = dht3;
            old_index = index;
        }

        // Ensure the vector has enough capacity before updating elements
        if dht.len() > 2 {
            dht[2] = (((index - 2) >> 8) & 0xFF) as u8;
        }
        if dht.len() > 3 {
            dht[3] = ((index - 2) & 0xFF) as u8;
        }

        writer.write_array(&dht);
    }
}
