/// Represents a structure for storing JPEG image metadata and parameters.
///
/// This structure holds information about the precision, dimensions, and components of a JPEG image,
/// as well as various tables and parameters used in the compression process.
#[derive(Debug)]
pub struct JpegObj {
    /// Precision of the image data in bits. Typically 8 bits for standard JPEG.
    pub precision: u8,

    /// Height of the image in pixels.
    pub image_height: u16,

    /// Width of the image in pixels.
    pub image_width: u16,

    /// Number of color components in the image (e.g., 3 for RGB).
    pub number_of_components: u8,

    /// Identifier for each color component.
    pub comp_id: Vec<u8>,

    /// Horizontal sampling factor for each color component.
    pub hsamp_factor: Vec<u8>,

    /// Vertical sampling factor for each color component.
    pub vsamp_factor: Vec<u8>,

    /// Quantization table number for each color component.
    pub qtable_number: Vec<u8>,

    /// DC Huffman table number for each color component.
    pub dctable_number: Vec<u8>,

    /// AC Huffman table number for each color component.
    pub actable_number: Vec<u8>,

    /// Start of spectral selection.
    pub ss: u8,

    /// End of spectral selection.
    pub se: u8,

    /// Successive approximation bit position high.
    pub ah: u8,

    /// Successive approximation bit position low.
    pub al: u8,
}

/// Implements the default constructor for `JpegObj`.
///
/// The default constructor initializes a new `JpegObj` with default values
/// representing a typical JPEG configuration.
impl Default for JpegObj {
    fn default() -> Self {
        JpegObj {
            precision: 8,
            image_height: 512,
            image_width: 512,
            number_of_components: 3,
            comp_id: vec![1, 2, 3],
            hsamp_factor: vec![1, 1, 1],
            vsamp_factor: vec![1, 1, 1],
            qtable_number: vec![0, 1, 1],
            dctable_number: vec![0, 1, 1],
            actable_number: vec![0, 1, 1],
            ss: 0,
            se: 63,
            ah: 0,
            al: 0,
        }
    }
}
