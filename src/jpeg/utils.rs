use crate::jpeg::comment::CommentHeader;
use crate::jpeg::dct::DctStruct;
use crate::jpeg::dht::DhtHeader;
use crate::jpeg::dqt::DqtHeader;
use crate::jpeg::header::JfifHeader;
use crate::jpeg::huff::Huffman;
use crate::jpeg::obj::JpegObj;
use crate::jpeg::sof::SofHeader;
use crate::jpeg::sos::SosHeader;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::SeekFrom;
use std::io::{BufReader, ErrorKind, Read, Seek};

// ANSI escape codes for text color
const COLOR_RED: &str = "\x1b[91m";
const COLOR_GREEN: &str = "\x1b[92m";
const COLOR_YELLOW: &str = "\x1b[93m";
const COLOR_RESET: &str = "\x1b[0m";

type JpegHeadersResult = Result<
    (
        Option<JfifHeader>,
        Option<CommentHeader>,
        Option<DqtHeader>,
        Option<SofHeader>,
        Option<DhtHeader>,
        Option<SosHeader>,
    ),
    Box<dyn Error>,
>;

/// Reads a 16-bit marker from the specified `Read` trait object.
///
/// The `read_marker` function attempts to read a 16-bit marker from the provided `Read` trait object.
/// It returns a result indicating either the successfully read marker value or an `io::Error` if
/// the read operation encounters an error, such as unexpected end-of-file.
///
/// # Arguments
///
/// * `reader` - A mutable reference to a type implementing the `Read` trait, from which the marker is read.
///
/// # Returns
///
/// A result containing either the 16-bit marker value or an `io::Error`.
///
/// If the marker is successfully read, it is returned as a `u16` using big-endian byte order.
/// If an error occurs during the read operation, the function returns an `io::Error`.
///
/// If an unexpected end-of-file error occurs, a warning message is printed to stderr, and the
/// function continues execution, returning a placeholder value of 0. You may choose to handle
/// this case differently by modifying the returned value in the placeholder section.
///
/// # Examples
///
/// ```
/// use std::io::{self, Cursor};
/// use stegano::jpeg::utils::read_marker;
///
/// let data = [0xFF, 0xDA]; // Example marker bytes
/// let mut reader = Cursor::new(&data);
///
/// match read_marker(&mut reader) {
///     Ok(marker) => {
///         println!("Successfully read marker: {:#04X}", marker);
///     }
///     Err(e) => {
///         eprintln!("Error reading marker: {}", e);
///     }
/// }
/// ```
pub fn read_marker(reader: &mut dyn Read) -> io::Result<u16> {
    let mut marker = [0u8; 2];

    match reader.read_exact(&mut marker) {
        Ok(_) => Ok(u16::from_be_bytes(marker)),
        Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
            // Print a message and continue with the loop
            eprintln!("Warning: Unexpected end of file while reading marker. Continuing...");
            Ok(0)
        }
        Err(e) => Err(e),
    }
}

/// Reads various JPEG headers from a file and returns them as a tuple of optional header structs.
///
/// The `read_jpeg_headers` function reads JPEG headers, including JFIF, Comment, DQT, SOF, DHT, and SOS headers,
/// from the specified file. It returns a tuple containing optional instances of the corresponding header structs.
/// If a header is not encountered in the file, the corresponding option in the tuple is `None`.
///
/// # Arguments
///
/// * `file_path` - A string slice representing the path to the JPEG file.
/// * `start_chunk` - The index of the starting chunk to read.
/// * `end_chunk` - The index of the ending chunk to read.
/// * `num_chunks` - The number of chunks to read in each iteration.
///
/// # Returns
///
/// A `Result` containing a tuple of optional header structs or an `io::Error` if an error occurs during the reading process.
///
/// The tuple elements represent the following JPEG headers:
/// - `JfifHeader`: JFIF (JPEG File Interchange Format) header information.
/// - `CommentHeader`: Comment header containing additional information.
/// - `DqtHeader`: Quantization table header.
/// - `SofHeader`: Start of Frame header.
/// - `DhtHeader`: Define Huffman Table header.
/// - `SosHeader`: Start of Scan header.
///
/// If a required header is missing, the function returns an error indicating the absence of the header.
///
/// # Examples
///
/// ```
/// use stegano::jpeg::utils::read_jpeg_headers;
/// use std::fs::File;
///
/// let output_file = File::create("temp.jpeg").unwrap();
///
/// match read_jpeg_headers("temp.jpeg", 0, 100, 10) {
///     Ok((jfif, comment, dqt, sof, dht, sos)) => {
///         // Process the obtained headers as needed
///     }
///     Err(e) => {
///         eprintln!("Error reading JPEG headers: {}", e);
///     }
/// }
/// ```
pub fn read_jpeg_headers(
    file_path: &str,
    start_chunk: usize,
    end_chunk: usize,
    num_chunks: usize,
) -> JpegHeadersResult {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut jfif_header = None;
    let mut dqt_header = None;
    let mut sof_header = None;
    let mut dht_header = None;
    let mut sos_header = None;
    let mut image_width = 0;
    let mut image_height = 0;
    let mut count_chunk = 0;

    let mut comment_data = None;
    let mut encountered_dqt = false;

    // Apply offset
    reader.seek(SeekFrom::Current(start_chunk as i64))?;
    for current_chunk in start_chunk..=end_chunk {
        let marker = read_marker(&mut reader)?;

        match marker {
            0xFFE0 => {
                // JFIF Marker
                let mut data_length_bytes = [0u8; 2];
                reader.read_exact(&mut data_length_bytes)?;
                let data_length = u16::from_be_bytes(data_length_bytes);
                let mut data = vec![0u8; data_length as usize + 2];
                reader.read_exact(&mut data)?;

                // Process data and store in the struct
                jfif_header = Some(JfifHeader::new(&data).unwrap());
                println!(
                    "{}JFIF Header: {:?}{}",
                    COLOR_GREEN,
                    jfif_header.clone().unwrap(),
                    COLOR_RESET
                );
            }
            0xFFFE => {
                // Comment Marker
                let mut data_length_bytes = [0u8; 2];
                reader.read_exact(&mut data_length_bytes)?;
                let data_length = u16::from_be_bytes(data_length_bytes);
                let mut data = vec![0u8; data_length as usize - 2];
                reader.read_exact(&mut data)?;
                comment_data = Some(data);
                println!(
                    "{}Comment Header: {:?}{}",
                    COLOR_YELLOW,
                    comment_data.clone().unwrap(),
                    COLOR_RESET
                );
            }
            0xFFDB => {
                // DQT Marker
                let mut data_length_bytes = [0u8; 2];
                reader.read_exact(&mut data_length_bytes)?;
                let data_length = u16::from_be_bytes(data_length_bytes);
                let mut data = vec![0u8; data_length as usize - 4];
                reader.read_exact(&mut data)?;

                // Process data and store in the struct
                let dct_struct = DctStruct::new(&data).unwrap();
                dqt_header = Some(DqtHeader::new(dct_struct));
                encountered_dqt = true;
                println!(
                    "{}DQT Header for Chunk#{}: {:?}{}",
                    COLOR_GREEN,
                    current_chunk,
                    dqt_header.clone().unwrap(),
                    COLOR_RESET
                );
            }
            0xFFC0 => {
                // SOF Marker
                let mut data_length_bytes = [0u8; 2];
                reader.read_exact(&mut data_length_bytes)?;
                let data_length = u16::from_be_bytes(data_length_bytes);
                let mut data = vec![0u8; data_length as usize - 2];
                reader.read_exact(&mut data)?;

                // Process data and store in the struct
                let jpeg_obj = process_sof_data(&data);
                sof_header = Some(SofHeader::new(jpeg_obj));
                image_width = sof_header.clone().unwrap().jpeg_obj.image_width;
                image_height = sof_header.clone().unwrap().jpeg_obj.image_height;
                println!(
                    "{}SOF Header for Chunk#{}: {:?}{}",
                    COLOR_YELLOW,
                    current_chunk,
                    sof_header.clone().unwrap(),
                    COLOR_RESET
                );
            }
            0xFFC4 => {
                // DHT Marker
                let mut data_length_bytes = [0u8; 2];
                reader.read_exact(&mut data_length_bytes)?;
                let data_length = u16::from_be_bytes(data_length_bytes);
                let mut data = vec![0u8; data_length as usize - 2];
                reader.read_exact(&mut data)?;

                // Process data and store in the struct
                let huf_struct = process_dht_data(&data);
                dht_header = Some(DhtHeader::new(huf_struct));
                println!(
                    "{}Processing DHT Header for Chunk#{}: {}",
                    COLOR_RED, current_chunk, COLOR_RESET
                );
            }
            0xFFDA => {
                // SOS Marker
                let mut data_length_bytes = [0u8; 2];
                reader.read_exact(&mut data_length_bytes)?;
                let data_length = u16::from_be_bytes(data_length_bytes);
                let mut data = vec![0u8; data_length as usize - 2];
                reader.read_exact(&mut data)?;

                // Process data and store in the struct
                let jpeg_obj = process_sos_data(&data, image_height, image_width);
                sos_header = Some(SosHeader::new(jpeg_obj));
                println!(
                    "{}SOS Header for Chunk#{}: {:?}{}",
                    COLOR_GREEN,
                    current_chunk,
                    sos_header.clone().unwrap(),
                    COLOR_RESET
                );
            }
            0xFFD9 => {
                // EOI Marker - End of Headers
                println!(
                    "{}End of Headers for Chunk {}{}",
                    COLOR_RED, current_chunk, COLOR_RESET
                );
                break;
            }
            0 => {
                // EOI Marker - End of Headers
                println!(
                    "{}End of Headers for Chunk {}{}",
                    COLOR_RED, current_chunk, COLOR_RESET
                );

                break;
            }
            _ => {
                // println!("{}Ignoring Marker {} for Chunk {}{}", COLOR_YELLOW, marker, current_chunk, COLOR_RESET);
                // Ignore other markers
            }
        }
        if current_chunk > end_chunk {
            // Stop reading after the specified end chunk
            break;
        }
        count_chunk += 1;
        if count_chunk > num_chunks {
            // Stop reading after the specified end chunk
            break;
        }
    }

    // Process comment data after the loop
    let comment_str = match comment_data {
        Some(data) => String::from_utf8_lossy(&data).trim().to_owned(),
        None => String::new(),
    };
    let comment_header = Some(CommentHeader::new(&comment_str));

    // Ensure all headers are present, and make headers optional
    Ok((
        jfif_header,
        comment_header,
        if encountered_dqt {
            Some(
                dqt_header
                    .ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "Missing DQT header"))?,
            )
        } else {
            None
        },
        sof_header,
        dht_header,
        sos_header,
    ))
}

/// Processes Start of Frame (SOF) data and populates a `JpegObj` struct with the extracted information.
///
/// The `process_sof_data` function takes a slice of raw SOF data and extracts information such as
/// precision, image height, image width, number of components, component IDs, horizontal and
/// vertical sampling factors, and quantization table numbers. It then populates a `JpegObj` struct
/// with the extracted data and returns it.
///
/// # Arguments
///
/// * `data` - A slice of raw SOF data representing the Start of Frame marker payload.
///
/// # Returns
///
/// A `JpegObj` struct populated with the extracted information from the SOF data.
///
/// # Examples
///
/// ```
/// use stegano::jpeg::utils::process_sof_data;
///
/// let sof_data: [u8; 11] = [8, 0, 100, 200, 3, 1, 2, 3, 10, 20, 30];
/// let jpeg_obj = process_sof_data(&sof_data);
/// ```
pub fn process_sof_data(data: &[u8]) -> JpegObj {
    let precision = data[0];
    let image_height = u16::from_be_bytes([data[1], data[2]]);
    let image_width = u16::from_be_bytes([data[3], data[4]]);
    let number_of_components = data[5];

    let mut comp_id = Vec::with_capacity(number_of_components as usize);
    let mut hsamp_factor = Vec::with_capacity(number_of_components as usize);
    let mut vsamp_factor = Vec::with_capacity(number_of_components as usize);
    let mut qtable_number = Vec::with_capacity(number_of_components as usize);

    let mut index = 6;

    for _ in 0..number_of_components {
        comp_id.push(data[index]);
        index += 1;
        hsamp_factor.push((data[index] >> 4) & 0xF);
        vsamp_factor.push(data[index] & 0xF);
        index += 1;
        qtable_number.push(data[index]);
        index += 1;
    }

    let dctable_number = (1..=number_of_components).collect();
    let actable_number = (11..=11 + number_of_components).collect();

    let ss = 0x00; // Start of spectral selection
    let se = 0x3F; // End of spectral selection
    let ah = 0x00; // Successive approximation bit position high
    let al = 0x00; // Successive approximation bit position low

    JpegObj {
        precision,
        image_height,
        image_width,
        number_of_components,
        comp_id,
        hsamp_factor,
        vsamp_factor,
        qtable_number,
        dctable_number,
        actable_number,
        ss,
        se,
        ah,
        al,
    }
}

/// Processes Define Huffman Table (DHT) data and populates a `Huffman` struct with the extracted information.
///
/// The `process_dht_data` function takes a slice of raw DHT data and extracts information such as
/// Huffman table bits, values, and metadata. It then populates a `Huffman` struct with the extracted
/// data and returns it.
///
/// # Arguments
///
/// * `data` - A slice of raw DHT data representing the Define Huffman Table marker payload.
///
/// # Returns
///
/// A `Huffman` struct populated with the extracted information from the DHT data.
///
/// The `Huffman` struct represents a Huffman table used for entropy encoding in JPEG compression.
/// It contains arrays for Huffman code lengths (`bits`) and the corresponding values (`val`).
///
/// # Examples
///
/// ```
/// use stegano::jpeg::utils::process_dht_data;
///
/// let dht_data: [u8; 50] = [0xFF, 0xC4, 0x00, 0x19, 0x00, 0x00, 0x01, 0x05, 0x01, 0x01, 0x01, 0x01,
///                           0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02,
///                           0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x10, 0x00, 0x02,
///                           0x01, 0x03, 0x03, 0x02, 0x04, 0x03, 0x05, 0x05, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00];
///
/// let huffman_table = process_dht_data(&dht_data);
/// ```
pub fn process_dht_data(data: &[u8]) -> Huffman {
    let image_height = u16::from_be_bytes([data[0], data[1]]);
    let image_width = u16::from_be_bytes([data[2], data[3]]);

    let mut huf_struct = Huffman::new(image_width as i32, image_height as i32);

    let mut index = 4;
    let mut old_index = 4;

    for i in 0..4 {
        if index < data.len() {
            let bytes = data[index];
            huf_struct.bits[i][0] = bytes as i32;
            index += 1;
        } else {
            // Handle the case where the index goes beyond the data length
            // eprintln!(
            //     "Warning: Index out of bounds when reading huf_struct.bits[{}].",
            //     i
            // );
            break;
        }

        for j in 1..17 {
            if index < data.len() {
                huf_struct.bits[i][j] = data[index] as i32;
                index += 1;
            } else {
                // Handle the case where the index goes beyond the data length
                // eprintln!(
                //     "Warning: Index out of bounds when reading huf_struct.bits[{}][{}].",
                //     i, j
                // );
                break;
            }
        }

        let bytes = huf_struct.bits[i][0] as usize;

        if index + bytes <= data.len() {
            let huf_vals: Vec<i32> = data[index..index + bytes]
                .iter()
                .map(|&x| x as i32)
                .collect();
            huf_struct.val[i] = huf_vals;
            index += bytes;
        } else {
            // eprintln!(
            //     "Warning: Index out of bounds when reading huf_vals for huf_struct.val[{}].",
            //     i
            // );
        }

        let mut dht3 = vec![0xFF, 0xC4];
        dht3.extend_from_slice(&data[old_index..index]);
        old_index = index;
        huf_struct.bits[i][2] = ((index - 2) >> 8) as i32;
        huf_struct.bits[i][3] = (index - 2) as i32;
    }
    huf_struct
}

/// Processes Start of Scan (SOS) data and populates a `JpegObj` struct with the extracted information.
///
/// The `process_sos_data` function takes a slice of raw SOS data and extracts information such as
/// precision, image height, image width, number of components, component IDs, Huffman table numbers,
/// sampling factors, and additional parameters. It then populates a `JpegObj` struct with the
/// extracted data and returns it.
///
/// # Arguments
///
/// * `data` - A slice of raw SOS data representing the Start of Scan marker payload.
///
/// # Returns
///
/// A `JpegObj` struct populated with the extracted information from the SOS data.
///
/// The `JpegObj` struct represents a set of parameters related to a JPEG image. It includes
/// information such as precision, image dimensions, number of components, component IDs, Huffman
/// table numbers, sampling factors, and other relevant details.
///
/// # Examples
///
/// ```
/// use stegano::jpeg::utils::process_sos_data;
///
/// let sos_data: [u8; 15] = [8, 0, 100, 200, 3, 1, 1, 0x11, 0x00, 0x2C, 0x00, 0x00, 0x00, 0x00, 0x00];
/// let jpeg_obj = process_sos_data(&sos_data, 10, 10);
/// ```
pub fn process_sos_data(data: &[u8], image_height: u16, image_width: u16) -> JpegObj {
    let precision = data[0];
    let number_of_components = data[5];

    let mut comp_id = Vec::with_capacity(number_of_components as usize);
    let mut dc_table_number = Vec::with_capacity(number_of_components as usize);
    let mut ac_table_number = Vec::with_capacity(number_of_components as usize);
    let mut hsamp_factor = Vec::with_capacity(number_of_components as usize);
    let mut vsamp_factor = Vec::with_capacity(number_of_components as usize);
    let mut qtable_number = Vec::with_capacity(number_of_components as usize);

    let mut index = 6;

    for _ in 0..number_of_components {
        if index + 1 < data.len() {
            comp_id.push(data[index]);
            index += 1;

            dc_table_number.push((data[index] >> 4) & 0xF);
            ac_table_number.push(data[index] & 0xF);
            index += 1;

            hsamp_factor.push((data[index] >> 4) & 0xF);
            vsamp_factor.push(data[index] & 0xF);
            qtable_number.push(data[index + 1]);

            index += 2;
        } else {
            // eprintln!("Warning: Index out of bounds when reading SOS component data.");
            break;
        }
    }

    let ss = if index < data.len() { data[index] } else { 0 };
    let se = if index + 1 < data.len() {
        data[index + 1]
    } else {
        0
    };
    let ah_al = if index + 2 < data.len() {
        data[index + 2]
    } else {
        0
    };
    let ah = (ah_al >> 4) & 0xF;
    let al = ah_al & 0xF;

    JpegObj {
        precision,
        image_height,
        image_width,
        number_of_components,
        comp_id,
        hsamp_factor,
        vsamp_factor,
        qtable_number,
        dctable_number: dc_table_number,
        actable_number: ac_table_number,
        ss,
        se,
        ah,
        al,
    }
}
