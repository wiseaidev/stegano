//! # stegano
//!
//! A powerful and versatile steganography tool designed to empower you with a wide range of image manipulation
//! and data encryption capabilities.
//!
//! # Quick Start
//!
//! Get started with the `stegano` crate by following these simple steps:
//!
//! 1. Install the `stegano` tool using Cargo:
//!
//! ```bash
//! cargo install --locked stegano
//! ```
//!
//! 2. Use the `stegano` subcommands to process and manipulate png images. Here are some examples:
//!
//! 1. Read and process 10 chunks from the image:
//!
//!    ```bash
//!    $ stegano show-meta -i image_file_name
//!    It is a valid PNG file. Let's process it!
//!    ---- Chunk #1 ----
//!    Chunk offset: 13
//!    Chunk size: 0
//!    Chunk crc: 3d008
//!    ---- Chunk #2 ----
//!    Chunk offset: 30
//!    Chunk size: 0
//!    Chunk crc: 52474200
//!    ---- Chunk #3 ----
//!    Chunk offset: 47
//!    Chunk size: 73
//!    Chunk crc: 333db36f
//!    ---- Chunk #4 ----
//!    Chunk offset: 137
//!    Chunk size: 91
//!    Chunk crc: 4000023e
//!    ---- Chunk #5 ----
//!    Chunk offset: 245
//!    Chunk size: 251
//!    Chunk crc: 21080
//!    ---- Chunk #6 ----
//!    Chunk offset: 513
//!    Chunk size: 108
//!    Chunk crc: 10021080
//!    ---- Chunk #7 ----
//!    Chunk offset: 638
//!    Chunk size: 2
//!    Chunk crc: 35025ded
//!    ---- Chunk #8 ----
//!    Chunk offset: 657
//!    Chunk size: 3
//!    Chunk crc: 25b3f696
//!    ---- Chunk #9 ----
//!    Chunk offset: 677
//!    Chunk size: 57
//!    Chunk crc: 80000420
//!    ---- Chunk #10 ----
//!    Chunk offset: 751
//!    Chunk size: 64
//!    Chunk crc: 91cf0867
//!    ```
//!
//! 2. Process the image in silent mode:
//!
//!    ```bash
//!    $ stegano show-meta -i image_file_name -s
//!    ```
//!
//! 3. Read chunks at different positions:
//!
//!    ```bash
//!    # Read 1 chunk starting from position 0
//!    $ stegano show-meta -i image_file_name -c 0 -u 10 -n 1
//!    It is a valid PNG file. Let's process it!
//!    ---- Chunk #0 ----
//!    Chunk offset: 13
//!    Chunk size: 0
//!    Chunk crc: 3d008
//!
//!    # Read 3 chunks starting from position 10000
//!    $ stegano show-meta -i image_file_name -c 10000 -u 200000 -n 3
//!    It is a valid PNG file. Let's process it!
//!    ---- Chunk #10000 ----
//!    Chunk offset: 13
//!    Chunk size: 0
//!    Chunk crc: 3d008
//!    ---- Chunk #10001 ----
//!    Chunk offset: 30
//!    Chunk size: 0
//!    Chunk crc: 52474200
//!    ---- Chunk #10002 ----
//!    Chunk offset: 47
//!    Chunk size: 73
//!    Chunk crc: 333db36f
//!    ```
//!
//! 4. Encrypt an inject data in a new image from a given image:
//!
//!    ```bash
//!    # Encode and inject a payload in a new output image from a given input image.
//!    $ stegano encrypt -i input_image_file_name -k 'pass' -p 'hello' -f 159028 -o output_image_file_name -s
//!    It is a valid PNG file. Let's process it!
//!    Image encoded and written successfully!
//!    ```
//!
//! 5. Decrypt, extract secrets from an image and remove the secret from the image:
//!
//!    ```bash
//!    $ stegano decrypt -i input_image_file_name -k 'pass' -f 159028 -o output_image_file_name -s
//!    Your decoded secret is: "hello"
//!    ```
//!
//! # Options
//!
//! | Option                  | Description                                               |
//! |-------------------------|-----------------------------------------------------------|
//! | **Encryption Options**  |                                                           |
//! | `-i` or `--input`       | Sets the input file for encryption.                        |
//! | `-o` or `--output`      | Sets the output file for the encrypted payload (default is "output.png").|
//! | `-k` or `--key`         | Sets the key for payload encryption (default is "key").    |
//! | `-s` or `--suppress`    | Suppresses output messages.                                |
//! | `-f` or `--offset`      | Sets the offset (default is 10).                           |
//! | `-p` or `--payload`     | Sets the payload (default is "hello").                     |
//! | `-t` or `--type`        | Sets the type (default is "PNG").                          |
//! |                         |                                                           |
//! | **Decryption Options**  |                                                           |
//! | `-i` or `--input`       | Sets the input file for decryption.                        |
//! | `-o` or `--output`      | Sets the output file for the decrypted payload (default is "output.png").|
//! | `-k` or `--key`         | Sets the key for payload decryption (default is "key").    |
//! | `-s` or `--suppress`    | Suppresses output messages.                                |
//! | `-f` or `--offset`      | Sets the offset (default is 10).                           |
//! | `-p` or `--payload`     | Sets the payload (default is "hello").                     |
//! | `-t` or `--type`        | Sets the type (default is "PNG").                          |
//! |                         |                                                           |
//! | **Metadata Options**    |                                                           |
//! | `-i` or `--input`       | Sets the input image file for metadata extraction.         |
//! | `-n` or `--nb-chunks`   | Read a specific number of chunks (default is 10).          |
//! | `-c` or `--start`       | Sets the index of the start chunk to read from (default 1). |
//! | `-u` or `--end`         | Sets the index of the end chunk to stop reading at (default 11).|
//! | `-s` or `--suppress`    | Suppresses output messages.                                |
//!
//! # GitHub Repository
//!
//! You can access the source code for this crate on [GitHub](https://github.com/wiseaidev/stegano).
//!
//! # Contributing
//!
//! Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement,
//! please engage with the project on [GitHub](https://github.com/wiseaidev/stegano).
//! Your contributions help improve this crate for the community.

pub mod cli;
pub mod jpeg;
pub mod models;
pub mod utils;
