use crate::cli::{DecryptCmd, EncryptCmd, ShowMetaCmd};
use crate::utils::{decrypt_data, print_hex, u64_to_u8_array, xor_encrypt_decrypt};
use std::fs::File;
use std::io::{copy, Error, ErrorKind, Read, Seek, SeekFrom, Write};
use std::mem;

/// Represents the header of a PNG format.
///
/// # Fields
///
/// - `header` - A 64-bit unsigned integer representing the PNG header.
///
/// # Examples
///
/// ```
/// use stegano::models::Header;
///
/// let png_header = Header { header: 0x8950_4E47_0D0A_1A0A };
/// println!("PNG Header: {:X}", png_header.header);
/// ```
#[derive(Debug, Clone)]
pub struct Header {
    /// A 64-bit unsigned integer representing the PNG header.
    pub header: u64,
}

/// Represents a generic chunk in the PNG format.
///
/// # Fields
///
/// - `size` - The size of the chunk data in bytes.
/// - `r#type` - A 32-bit unsigned integer representing the chunk type.
/// - `data` - A vector of bytes containing the chunk data.
/// - `crc` - A 32-bit unsigned integer representing the cyclic redundancy check value for the chunk.
///
/// # Examples
///
/// ```
/// use stegano::models::Chunk;
///
/// let png_chunk = Chunk {
///     size: 13,
///     r#type: 0x4949_4444,
///     data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
///     crc: 0xABCD_EF01,
/// };
/// println!("Chunk Type: {:X}", png_chunk.r#type);
/// ```
#[derive(Debug, Clone)]
pub struct Chunk {
    /// The size of the chunk data in bytes.
    pub size: u32,
    /// A 32-bit unsigned integer representing the chunk type.
    pub r#type: u32,
    /// A vector of bytes containing the chunk data.
    pub data: Vec<u8>,
    /// A 32-bit unsigned integer representing the cyclic redundancy check value for the chunk.
    pub crc: u32,
}

/// Represents a meta chunk in the PNG format, composed of a header and a generic chunk.
///
/// # Fields
///
/// - `header` - The header of the meta chunk.
/// - `chk` - A generic chunk representing the meta chunk data.
/// - `offset` - A 64-bit unsigned integer representing the offset of the meta chunk.
///
/// # Examples
///
/// ```
/// use stegano::models::{MetaChunk, Chunk, Header};
///
/// let meta_chunk = MetaChunk {
///     header: Header { header: 0x8950_4E47_0D0A_1A0A },
///     chk: Chunk {
///         size: 13,
///         r#type: 0x4949_4444,
///         data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
///         crc: 0xABCD_EF01,
///     },
///     offset: 42,
/// };
/// println!("Meta Chunk Offset: {}", meta_chunk.offset);
/// ```
#[derive(Debug, Clone)]
pub struct MetaChunk {
    /// The header of the meta chunk.
    pub header: Header,
    /// A generic chunk representing the meta chunk data.
    pub chk: Chunk,
    /// A 64-bit unsigned integer representing the offset of the meta chunk.
    pub offset: u64,
}

impl MetaChunk {
    /// Pre-processes a PNG image file to extract the PNG header and initializes a MetaChunk.
    ///
    /// This function reads the PNG header from the provided file, validates it, and creates
    /// a MetaChunk with an empty Chunk and the offset set to the current position in the file.
    ///
    /// # Arguments
    ///
    /// - `file` - A mutable reference to a File representing the PNG image file.
    /// - `suppress`: A boolean to suppress print statements.
    ///
    /// # Returns
    ///
    /// A Result containing the initialized MetaChunk if successful, or an Error if any
    /// _errors occur during the process.
    ///
    /// # Panics
    ///
    /// Panics if the file is not a valid PNG format.
    pub fn new(file: &mut File, suppress: bool) -> Result<MetaChunk, Error> {
        let mut header = Header { header: 0 };
        file.read_exact(unsafe { mem::transmute::<_, &mut [u8; 8]>(&mut header.header) })?;
        let b_arr = u64_to_u8_array(header.header);
        let offset = file.stream_position()?;
        if &b_arr[1..4] != b"PNG" {
            let _err = Error::new(ErrorKind::Other, "Not a valid PNG file!");
            return Err(_err);
        } else if !suppress {
            println!("It is a valid PNG file. Let's process it! \n");
            // print header
            println!("\x1b[92m---- Header ----\x1b[0m");
            print_hex(&b_arr, 0);
            print!("\x1b[0m");
            println!("\x1b[92m----- End ------\x1b[0m");
            println!();
        }
        Ok(MetaChunk {
            header,
            chk: Chunk {
                size: 0,
                r#type: 0,
                data: Vec::new(),
                crc: 0,
            },
            offset,
        })
    }

    /// Processes a PNG image file by reading and displaying information about its chunks.
    ///
    /// This function iterates through the chunks in the provided file, printing information
    /// about each chunk, until the 'IEND' chunk is encountered.
    ///
    /// # Arguments
    ///
    /// - `file` - A mutable reference to a File representing the PNG image file.
    /// - `c`: A reference to `ShowMetaCmd` containing command-line arguments.
    pub fn process_image(&mut self, file: &mut File, c: &ShowMetaCmd) {
        let mut start_position: usize = c.start_chunk;
        let mut end_position: usize = c.end_chunk;
        let mut _chunk_type = String::new();
        let end_chunk_type = "IEND";
        if c.read_end {
            file.seek(SeekFrom::End(
                (-(start_position as isize)).try_into().unwrap(),
            ))
            .unwrap();
            start_position = file.metadata().unwrap().len() as usize - c.nb_chunks;
            end_position = file.metadata().unwrap().len() as usize - 1;
        } else {
            file.seek(SeekFrom::Start((start_position).try_into().unwrap()))
                .unwrap();
            if c.start_chunk > 8 {
                self.offset = start_position as u64;
            }
        }
        for (i, j) in (start_position..end_position).enumerate() {
            _chunk_type = self.chunk_type_to_string();
            if i >= c.nb_chunks || _chunk_type == end_chunk_type {
                break;
            }
            self.read_chunk(file);
            if !c.suppress {
                println!("\x1b[92m---- Chunk #{} ----\x1b[0m", j);
                println!("Offset: {:?}", self.offset);
                println!("Size: {:?}", self.chk.size);
                println!("CRC: {:x}", self.chk.crc);
                print_hex(&self.chk.data, self.offset);
                print!("\x1b[0m");
                println!("\x1b[92m------- End -------\x1b[0m");
                println!();
            }
            let _offset = self.get_offset(file);
        }
    }

    /// Gets the offset from the current position in the file and updates the MetaChunk offset.
    ///
    /// This function reads the offset from the current position in the file, updates the offset
    /// field in the MetaChunk, and returns the obtained offset.
    ///
    /// # Arguments
    ///
    /// - `file` - A mutable reference to a type implementing Read and Seek.
    ///
    /// # Returns
    ///
    /// The offset obtained from the current position in the file.
    fn get_offset<T: Read + Seek>(&mut self, file: &mut T) -> u64 {
        let offset = file.stream_position().unwrap();
        self.offset = offset;
        offset
    }

    /// Reads a PNG chunk from the provided file and updates the MetaChunk.
    ///
    /// This function reads the size, type, data, and CRC of a PNG chunk from the file,
    /// updates the fields in the MetaChunk, and advances the file cursor accordingly.
    ///
    /// # Arguments
    ///
    /// - `file` - A mutable reference to a type implementing Read and Seek.
    fn read_chunk<T: Read + Seek>(&mut self, file: &mut T) {
        self.read_chunk_size(file);
        self.read_chunk_type(file);
        self.read_chunk_bytes(file, self.chk.size);
        self.read_chunk_crc(file);
    }

    /// Reads the size of a PNG chunk from the provided file and updates the Chunk size.
    ///
    /// This function reads the size field of a PNG chunk from the file and updates the
    /// size field in the associated Chunk.
    ///
    /// # Arguments
    ///
    /// - `file` - A mutable reference to a type implementing Read.
    fn read_chunk_size<R: Read>(&mut self, file: &mut R) {
        let mut size_bytes = [0; 4];

        match file.read_exact(&mut size_bytes) {
            Ok(_) => {
                // Successfully read the expected number of bytes
                self.chk.size = u32::from_be_bytes(size_bytes);
                if self.chk.size > 40 {
                    let min_non_zero_number = *size_bytes
                        .iter()
                        .filter(|&&byte| byte != 0)
                        .min_by(|a, b| a.cmp(b))
                        .unwrap_or(&0);
                    self.chk.size = min_non_zero_number as u32;
                }
                // self.chk.size = size_bytes[3] as u32;
            }
            Err(_err) if _err.kind() == ErrorKind::UnexpectedEof => {
                // Handle the situation where the file ends before reading the expected bytes
                // eprintln!("Warning: Reached end of file prematurely while reading chunk size");
            }
            Err(_err) => {
                // eprintln!("Error reading chunk size bytes: {}", _err);
            }
        }
    }

    /// Reads the type of a PNG chunk from the provided file and updates the Chunk type.
    ///
    /// This function reads the type field of a PNG chunk from the file and updates the
    /// type field in the associated Chunk.
    ///
    /// # Arguments
    ///
    /// - `file` - A mutable reference to a type implementing Read.
    fn read_chunk_type<R: Read>(&mut self, file: &mut R) {
        let mut type_bytes = [0; 4];

        match file.read_exact(&mut type_bytes) {
            Ok(_) => {
                // Successfully read the expected number of bytes
                self.chk.r#type = u32::from_be_bytes(type_bytes);
            }
            Err(_err) if _err.kind() == ErrorKind::UnexpectedEof => {
                // Handle the situation where the file ends before reading the expected bytes
                // eprintln!("Warning: Reached end of file prematurely while reading chunk type");
            }
            Err(_err) => {
                // eprintln!("Error reading chunk type bytes: {}", _err);
            }
        }
    }

    /// Reads the data bytes of a PNG chunk from the provided file and updates the Chunk data.
    ///
    /// This function reads the data bytes of a PNG chunk from the file, updates the
    /// data field in the associated Chunk, and handles the situation where the file ends
    /// before reading the expected bytes.
    ///
    /// # Arguments
    ///
    /// - `file` - A mutable reference to a type implementing Read and Seek.
    /// - `len` - The expected length of the data in bytes.
    fn read_chunk_bytes<T: Read + Seek>(&mut self, file: &mut T, len: u32) {
        self.chk.data = vec![0; len as usize];

        match file.read_exact(&mut self.chk.data) {
            Ok(_) => {
                // Successfully read the expected number of bytes
            }
            Err(_err) if _err.kind() == ErrorKind::UnexpectedEof => {
                // eprintln!("Error reading chunk bytes: Reached end of file prematurely");
                // Update the length of the Chunk based on the actual number of bytes read
                self.chk
                    .data
                    .truncate(file.stream_position().unwrap() as usize);
            }
            Err(_err) => {
                // eprintln!("Error reading chunk bytes: {}", _err);
            }
        }
    }

    /// Reads the CRC field of a PNG chunk from the provided file and updates the Chunk CRC.
    ///
    /// This function reads the CRC field of a PNG chunk from the file and updates the
    /// crc field in the associated Chunk.
    ///
    /// # Arguments
    ///
    /// - `file` - A mutable reference to a type implementing Read.
    fn read_chunk_crc<R: Read>(&mut self, file: &mut R) {
        let mut crc_bytes = [0; 4];

        match file.read_exact(&mut crc_bytes) {
            Ok(_) => {
                // Successfully read the expected number of bytes
                self.chk.crc = u32::from_be_bytes(crc_bytes);
            }
            Err(_err) if _err.kind() == ErrorKind::UnexpectedEof => {
                // Handle the situation where the file ends before reading the expected bytes
                // eprintln!("Warning: Reached end of file prematurely while reading CRC");
            }
            Err(_err) => {
                // eprintln!("Error reading CRC bytes: {}", _err);
            }
        }
    }

    /// Converts the type field of the associated Chunk to a string representation.
    ///
    /// This function converts the type field of the associated Chunk to a string
    /// representation using utf8_lossy.
    ///
    /// # Returns
    ///
    /// A String representing the type of the associated Chunk.
    fn chunk_type_to_string(&self) -> String {
        String::from_utf8_lossy(&self.chk.r#type.to_be_bytes()).to_string()
    }

    /// Marshals the data of the associated Chunk into a vector of bytes.
    ///
    /// This function creates a vector of bytes containing the size, type, data, and CRC
    /// of the associated Chunk.
    ///
    /// # Returns
    ///
    /// A vector of bytes containing the marshaled data of the associated Chunk.
    fn marshal_data(&self) -> Vec<u8> {
        let mut bytes_msb = Vec::new();
        bytes_msb.push(self.chk.data.len() as u8);
        bytes_msb.write_all(&self.chk.r#type.to_be_bytes()).unwrap();
        bytes_msb.write_all(&self.chk.data).unwrap();
        bytes_msb.write_all(&self.chk.crc.to_be_bytes()).unwrap();
        bytes_msb
    }

    /// Writes data to a specified writer by encryption.
    ///
    /// This function takes a readable and seekable input, command arguments, and a writable output. It performs encryption
    /// processes based on the provided `EncryptCmd`. It encrypt the data using specific operations. The function uses the
    /// provided writer to output the processed data.
    ///
    /// # Arguments
    ///
    /// - `self`: A mutable reference to the instance of the struct containing this method.
    /// - `r`: A mutable reference to a readable and seekable input implementing `Read` and `Seek` traits.
    /// - `c`: A reference to `EncryptCmd` containing command-line arguments that determine  the encryption options.
    /// - `w`: A generic writable output implementing the `Write` trait.
    pub fn write_encrypted_data<R: Read + Seek, W: Write>(
        &mut self,
        r: &mut R,
        c: &EncryptCmd,
        mut w: W,
    ) {
        let b_arr = u64_to_u8_array(self.header.header);
        w.write_all(&b_arr).unwrap();
        let mut offset = c.offset;

        let encrypted_data = self.chk.data.clone();
        let encrypted_data_len = self.chk.data.len();
        let encrypted_data_crc = self.chk.crc;
        let init_position = r.stream_position().unwrap();
        if offset == 9999999999 {
            // Auto inject at IEND - 11
            // Read untill IEND
            offset = self.find_iend_offset(r);
            r.seek(SeekFrom::Start(init_position)).unwrap();
        }

        self.chk.data = encrypted_data.clone();
        self.chk.size = encrypted_data_len as u32;
        self.chk.crc = encrypted_data_crc;

        if !c.suppress {
            println!("\x1b[92m------- Chunk -------\x1b[0m");
            println!("Offset: {:?}", offset);
            println!("Size: {:?}", encrypted_data_len);
            println!("CRC: {:x}", encrypted_data_crc);
            print_hex(&encrypted_data, offset.try_into().unwrap());
            print!("\x1b[0m");
            println!("\x1b[92m-------- End --------\x1b[0m");
            println!();
        }
        let mut buff = vec![0; offset - 8];
        buff.resize(&offset - 8, 0);
        r.read_exact(&mut buff).unwrap();
        w.write_all(&buff).unwrap();
        let data: Vec<u8> = self.marshal_data();
        w.write_all(&data).unwrap();
        copy(r, &mut w).unwrap();
        println!(
            "Your payload has been encrypted and written at offset {} successfully!",
            offset
        );
    }

    /// Writes data to a specified writer by decryption.
    ///
    /// This function takes a readable and seekable input, command arguments, and a writable output. It performs decryption
    /// processes based on the provided `DecryptCmd`. It decrypt the data using specific operations. The function uses the
    /// provided writer to output the processed data.
    ///
    /// # Arguments
    ///
    /// - `self`: A mutable reference to the instance of the struct containing this method.
    /// - `r`: A mutable reference to a readable and seekable input implementing `Read` and `Seek` traits.
    /// - `c`: A reference to `DecryptCmd` containing command-line arguments that determine the decryption options.
    /// - `w`: A generic writable output implementing the `Write` trait.
    pub fn write_decrypted_data<R: Read + Seek, W: Write>(
        &mut self,
        r: &mut R,
        c: &DecryptCmd,
        mut w: W,
    ) {
        let b_arr = u64_to_u8_array(self.header.header);
        w.write_all(&b_arr).unwrap();
        let mut offset = c.offset;
        let init_position = r.stream_position().unwrap();
        if offset == 9999999999 {
            // Read untill IEND
            offset = self.find_iend_offset(r);
            r.seek(SeekFrom::Start(init_position)).unwrap();
        }
        let mut buff = vec![0; offset - 8];

        buff.resize(offset - 16, 0);
        r.read_exact(&mut buff).unwrap();
        w.write_all(&buff).unwrap();
        self.offset = r.seek(SeekFrom::Current(5)).unwrap();
        self.read_chunk(r);
        let mut decrypted_data: Vec<u8> = vec![0];
        match (*c.algorithm.to_lowercase()).into() {
            "aes" => {
                decrypted_data = decrypt_data(&c.key, &self.chk.data);
            }
            "xor" => {
                decrypted_data = xor_encrypt_decrypt(&self.chk.data, &c.key);
            }
            _ => {}
        }

        let decoded_string = String::from_utf8_lossy(&decrypted_data);
        let unpadded_string = decoded_string.trim_end_matches('\0');
        if !c.suppress {
            println!("\x1b[92m------- Chunk -------\x1b[0m");
            println!("Offset: {:?}", self.offset);
            println!("Size: {:?}", self.chk.size);
            println!("CRC: {:x}", self.chk.crc);
            print_hex(&decrypted_data, offset.try_into().unwrap());
            print!("\x1b[0m");
            println!("\x1b[92m-------- End --------\x1b[0m");
            println!();
        }
        r.seek(SeekFrom::Current(self.chk.data.len().try_into().unwrap()))
            .expect("Error seeking to offset");
        println!(
            "\x1b[38;5;7mYour decrypted secret is:\x1b[0m \x1b[38;5;214m{:?}\x1b[0m",
            unpadded_string
        );
        copy(r, &mut w).unwrap();
    }

    /// Finds the length of a file given a Read + Seek object.
    ///
    /// This function takes a readable and seekable input implementing both the `Read` and `Seek` traits.
    /// It saves the current position, moves the cursor to the end of the file to determine its length,
    /// and then restores the cursor to the saved position. The function returns the length of the file.
    ///
    /// # Arguments
    ///
    /// - `self`: A mutable reference to the instance of the struct containing this method.
    /// - `file`: A mutable reference to a readable and seekable input.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the length of the file if successful, or an `std::io::Error` if an error occurs.
    pub fn find_file_length<T>(&mut self, file: &mut T) -> std::io::Result<u64>
    where
        T: Read + Seek,
    {
        // Save the current position
        let current_position = file.stream_position()?;

        // Move the cursor to the end of the file
        let file_length = file.seek(SeekFrom::End(0))?;

        // Move the cursor back to the saved position
        file.seek(SeekFrom::Start(current_position))?;

        Ok(file_length)
    }

    /// Finds the offset of the last occurrence of the "IEND" chunk.
    ///
    /// This function takes a readable and seekable input implementing both the `Read` and `Seek` traits.
    /// It iterates through the chunks in the file until it reaches the "IEND" chunk, capturing the offset
    /// of the last occurrence. The offset is then adjusted for the chunk size, returning the final offset.
    ///
    /// # Arguments
    ///
    /// - `self`: A mutable reference to the instance of the struct containing this method.
    /// - `r`: A mutable reference to a readable and seekable input.
    ///
    /// # Returns
    ///
    /// Returns the offset of the last occurrence of the "IEND" chunk.
    fn find_iend_offset<R>(&mut self, r: &mut R) -> usize
    where
        R: Seek + Read,
    {
        let mut iend_offset = 999;
        let end_chunk_type = "IEND";

        while iend_offset < self.find_file_length(r).unwrap() {
            iend_offset = self.get_offset(r);
            self.read_chunk(r);
            let chunk_type = self.chunk_type_to_string();
            if chunk_type == end_chunk_type {
                break;
            }
        }

        (iend_offset - 11) as usize
    }
}
