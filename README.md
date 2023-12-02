# 🕵️‍♂️ Stegano

[![Crates.io](https://img.shields.io/crates/v/stegano.svg)](https://crates.io/crates/stegano)
[![Crates.io Downloads](https://img.shields.io/crates/d/stegano)](https://crates.io/crates/stegano)
![Rust](https://img.shields.io/badge/rust-stable-orange)
[![License](https://img.shields.io/crates/l/stegano.svg)](https://opensource.org/licenses/MIT)

> 🚀 `stegano`: Stegano is a powerful and versatile steganography tool designed to empower you with a wide range of image manipulation and data encryption capabilities.

## 📖 Table of Contents

- [Installation](#-installation)
- [Usage](#-usage)
- [Options](#-options)
- [Contributing](#-contributing)
- [License](#-license)

## 🚀 Installation

To install `stegano`, use the following Cargo command:

```bash
cargo install --locked stegano
```

## 🛠️ Usage

Use the `stegano` command to process and manipulate png images. Here are some examples:

1. Read and process 10 chunks from the image: 

   ```bash
   $ stegano -i image_file_name -m
   It is a valid PNG file. Let's process it!
   ---- Chunk #1 ----
   Chunk offset: 13
   Chunk size: 0
   Chunk crc: 3d008
   ---- Chunk #2 ----
   Chunk offset: 30
   Chunk size: 0
   Chunk crc: 52474200
   ---- Chunk #3 ----
   Chunk offset: 47
   Chunk size: 73
   Chunk crc: 333db36f
   ---- Chunk #4 ----
   Chunk offset: 137
   Chunk size: 91
   Chunk crc: 4000023e
   ---- Chunk #5 ----
   Chunk offset: 245
   Chunk size: 251
   Chunk crc: 21080
   ---- Chunk #6 ----
   Chunk offset: 513
   Chunk size: 108
   Chunk crc: 10021080
   ---- Chunk #7 ----
   Chunk offset: 638
   Chunk size: 2
   Chunk crc: 35025ded
   ---- Chunk #8 ----
   Chunk offset: 657
   Chunk size: 3
   Chunk crc: 25b3f696
   ---- Chunk #9 ----
   Chunk offset: 677
   Chunk size: 57
   Chunk crc: 80000420
   ---- Chunk #10 ----
   Chunk offset: 751
   Chunk size: 64
   Chunk crc: 91cf0867
   ```

1. Process the image in silent mode: 

   ```bash
   $ stegano -i image_file_name -m
   ```

1. Read chunks at different positions: 

   ```bash
   # Read 1 chunk starting from position 0
   $ stegano -i image_file_name -c 0 -u 10 -n 1 -m
   It is a valid PNG file. Let's process it!
   ---- Chunk #10000 ----
   Chunk offset: 13
   Chunk size: 0
   Chunk crc: 3d008

   # Read 3 chunks starting from position 10000
   $ stegano -i image_file_name -c 10000 -u 200000 -n 3 -m
   It is a valid PNG file. Let's process it!
   ---- Chunk #10000 ----
   Chunk offset: 13
   Chunk size: 0
   Chunk crc: 3d008
   ---- Chunk #10001 ----
   Chunk offset: 30
   Chunk size: 0
   Chunk crc: 52474200
   ---- Chunk #10002 ----
   Chunk offset: 47
   Chunk size: 73
   Chunk crc: 333db36f
   ```

1. Encode an inject data in an image: 

   ```bash
   # Encode and inject a payload in a new output image from a given input iamge. 
   $ stegano -i input_image_file_name -e -k 'pass' -p 'hello' -f 159028 -j -o output_image_file_name
   It is a valid PNG file. Let's process it!
   Image encoded and written successfully!
   ```

1. Decode, extract secrets from an image and remove the secret from the image: 

   ```bash
   $ stegano -i input_image_file_name -d -k 'pass' -j -f 159028 -o output_image_file_name -s
   Your decoded secret is: "hello"
   ```
## 🎨 Options

| Option                  | Description                                               |
|-------------------------|-----------------------------------------------------------|
| `-i` or `--input`       | Sets the input image file.                                 |
| `-o` or `--output`      | Sets the output file (default is "output.png").            |
| `-m` or `--meta`        | Enables metadata extraction.                               |
| `-n` or `--nb-chunks`   | Read a specific number of chunks (default is 10).          |
| `-c` or `--start`       | Sets the index of the start chunk to read from (default 1). |
| `-u` or `--end`         | Sets the index of the end chunk to stop reading at (default 11).|
| `-s` or `--suppress`    | Suppresses output messages.                                |
| `-f` or `--offset`      | Sets the offset (default is 10).                           |
| `-j` or `--inject`      | Enables injection.                                        |
| `-p` or `--payload`     | Sets the payload (default is "hello").                     |
| `-t` or `--type`        | Sets the type (default is "PNG").                          |
| `-e` or `--encode`      | Enables encoding.                                         |
| `-d` or `--decode`      | Enables decoding.                                         |
| `-k` or `--key`         | Sets the key for payload encryption (default is "key").    |

## 🤝 Contributing

Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement, please engage with the project on [GitHub](https://github.com/wiseaidev/stegano).
Your contributions help improve this crate for the community.

## 📄 License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
