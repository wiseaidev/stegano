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

`stegano` provides three subcommands:

```bash
$ stegano -h

The ultimate steganography swiss knife army tool.

Usage: stegano [COMMAND]

Commands:
  encrypt    Subcommand for encryption
  decrypt    Subcommand for decryption
  show-meta  Subcommand for showing metadata
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

```bash
$ stegano encrypt -h

Subcommand for encryption

Usage: stegano encrypt [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>      Sets the input file for injecting the payload
  -o, --output <OUTPUT>    Sets the output file for generating a new file with the injected payload [default: output.png]
  -k, --key <KEY>          Sets the key for payload encryption [default: key]
  -s, --suppress           Suppresses output messages
  -f, --offset <OFFSET>    Sets the offset [default: 10]
  -p, --payload <PAYLOAD>  Sets the payload [default: hello]
  -t, --type <TYPE>        Sets the type [default: PNG]
  -h, --help               Print help
  -V, --version            Print version

```

```bash
$ stegano decrypt -h
Subcommand for decryption

Usage: stegano decrypt [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>      Sets the input file for decrypting and extracting the payload
  -o, --output <OUTPUT>    Sets the output file for generating a new file with no payload, aka restoring the original file [default: output.png]
  -k, --key <KEY>          Sets the key for payload encryption [default: key]
  -s, --suppress           Suppresses output messages
  -f, --offset <OFFSET>    Sets the offset [default: 10]
  -p, --payload <PAYLOAD>  Sets the payload [default: hello]
  -t, --type <TYPE>        Sets the type [default: PNG]
  -h, --help               Print help
  -V, --version            Print version
```

```bash
$ stegano show-meta -h

Subcommand for showing metadata

Usage: stegano show-meta [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>          Sets the image input file
  -n, --nb-chunks <NB_CHUNKS>  Read number of chunks [default: 100]
  -s, --start <START_CHUNK>    The index of the start chunk to read from [default: 0]
  -e, --end <END_CHUNK>        The index of the end chunk to stop reading at [default: 100]
  -r, --suppress               Suppresses output messages
  -t, --type <TYPE>            Sets the type [default: PNG]
  -h, --help                   Print help
  -V, --version                Print version
```

Use these subcommands to process and manipulate png and jpeg images. Here are some examples:

1. Read and process 10 chunks from a png image: 

   ```bash
   $ stegano show-meta -i image_file_name -n 10
   It is a valid PNG file. Let's process it!
   ---- Chunk #0 ----
   Chunk offset: 13
   Chunk size: 0
   Chunk crc: 3e508
   ---- Chunk #1 ----
   Chunk offset: 30
   Chunk size: 0
   Chunk crc: 52474200
   ---- Chunk #2 ----
   Chunk offset: 47
   Chunk size: 103
   Chunk crc: ce4b5efe
   ---- Chunk #3 ----
   Chunk offset: 167
   Chunk size: 233
   Chunk crc: 62f481df
   ---- Chunk #4 ----
   Chunk offset: 417
   Chunk size: 143
   Chunk crc: 76626c80
   ---- Chunk #5 ----
   Chunk offset: 577
   Chunk size: 119
   Chunk crc: c44dff8c
   ---- Chunk #6 ----
   Chunk offset: 713
   Chunk size: 241
   Chunk crc: ef7257f9
   ---- Chunk #7 ----
   Chunk offset: 971
   Chunk size: 39
   Chunk crc: 222fa6b7
   ---- Chunk #8 ----
   Chunk offset: 1027
   Chunk size: 17
   Chunk crc: f63ff099
   ---- Chunk #9 ----
   Chunk offset: 1061
   Chunk size: 107
   Chunk crc: b01956ae
   ```

1. Process the png image in silent mode: 

   ```bash
   $ stegano show-meta -i image_file_name -r
   ```

1. Read and process a jpeg image.
   ```bash
   $ stegano show-meta -i image_file_name -t jpeg
   JFIF Header: JfifHeader { version: 18758 }
   SOF Header for Chunk#67: SofHeader { jpeg_obj: JpegObj { precision: 8, image_height: 243, image_width: 207, number_of_components: 3, comp_id: [1, 2, 3], hsamp_factor: [2, 1, 1], vsamp_factor: [2, 1, 1], qtable_number: [0, 1, 1], dctable_number: [1, 2, 3], actable_number: [11, 12, 13, 14], ss: 0, se: 63, ah: 0, al: 0 } }
   Processing DHT Header for Chunk#68: 
   Processing DHT Header for Chunk#69: 
   Processing DHT Header for Chunk#70: 
   Processing DHT Header for Chunk#71: 
   SOS Header for Chunk#72: SosHeader { jpeg_obj: JpegObj { precision: 3, image_height: 243, image_width: 207, number_of_components: 3, comp_id: [17], hsamp_factor: [3], vsamp_factor: [15], qtable_number: [0], dctable_number: [0], actable_number: [0], ss: 0, se: 0, ah: 0, al: 0 } }
   ```

   Sometimes the `JFIF` header doesn't exist, but it is still a valid jpeg file:

   ```bash
   $ stegano show-meta -i image_file_name -t jpeg
   DQT Header for Chunk#1: DqtHeader { dct: DctStruct { quantum: [[8, 2054, 1542, 1543, 1798, 1541, 1288, 2055, 1799, 1799, 1801, 2313, 2312, 2058, 2572, 3092, 5133, 3340, 3083, 2827, 2828, 3097, 6418, 4627, 4879, 3860, 5149, 7450, 6687, 7966, 7709, 7450, 6684, 7196, 7200, 8228, 9262, 11815, 10016, 8226, 8748, 11299, 8988, 7196, 7208, 10295, 14121, 10540, 11312, 12337, 12596, 13364, 13364, 13343, 7975, 10041, 14653, 15672, 14386, 12860, 15406, 11827, 13108, 13362], [12801, 265, 2313, 2313, 2316, 3083, 2828, 3096, 6157, 3341, 3352, 6194, 12833, 8476, 7201, 8498, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 12850, 0]] } }
   SOF Header for Chunk#3: SofHeader { jpeg_obj: JpegObj { precision: 8, image_height: 460, image_width: 460, number_of_components: 3, comp_id: [1, 2, 3], hsamp_factor: [2, 1, 1], vsamp_factor: [2, 1, 1], qtable_number: [0, 1, 1], dctable_number: [1, 2, 3], actable_number: [11, 12, 13, 14], ss: 0, se: 63, ah: 0, al: 0 } }
   Processing DHT Header for Chunk#4: 
   SOS Header for Chunk#5: SosHeader { jpeg_obj: JpegObj { precision: 3, image_height: 460, image_width: 460, number_of_components: 3, comp_id: [17], hsamp_factor: [3], vsamp_factor: [15], qtable_number: [0], dctable_number: [0], actable_number: [0], ss: 0, se: 0, ah: 0, al: 0 } }
   ```

1. Read chunks at different positions: 

   ```bash
   # Read 1 chunk starting from position 0
   $ stegano show-meta -i image_file_name -s 0 -e 10 -n 1
   It is a valid PNG file. Let's process it!
   ---- Chunk #0 ----
   Chunk offset: 13
   Chunk size: 0
   Chunk crc: 3d008

   # Read 3 chunks starting from position 10000
   $ stegano show-meta -i image_file_name -s 10000 -e 200000 -n 3
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

1. Encrypt an inject data in an image: 

   ```bash
   $ stegano encrypt -i input_image_file_name -k 'pass' -p 'hello' -f 159028 -o output_image_file_name -s
   It is a valid PNG file. Let's process it!
   Image encoded and written successfully!
   ```

1. Decrypt, extract secrets from an image and remove the secret from the image: 

   ```bash
   $ stegano decrypt -i input_image_file_name -k 'pass' -f 159028 -o output_image_file_name -s
   Your decoded secret is: "hello"
   ```

1. if wrong password is provided: 

   ```bash
   $ stegano decrypt -i input_image_file_name -k 'invalid' -f 159028 -o output_image_file_name -s
   Your decoded secret is: "qji~s"
   ```

1. if wrong password and wrong offset is provided: 

   ```bash
   $ stegano decrypt -i input_image_file_name -k 'invalid' -f 159020 -o output_image_file_name -s
   Your decoded secret is: "���dlidivr~sv\tE�\0DՁ\\�TAA\u{19}x�Z\u{7f}gp�\u{14}h��"
   ```

1. if correct password and wrong offset is provided: 

   ```bash
   $ stegano decrypt -i input_image_file_name -k 'pass' -f 159020 -o output_image_file_name -s
   Your decoded secret is: "���vpasshello\u{c}_�\u{6}DʛH�IDP\u{14}b�Cpbb�\u{1c}\u{7f}��"
   ```

## 🎨 Options

| Option                  | Description                                               |
|-------------------------|-----------------------------------------------------------|
| **Encryption Options**  |                                                           |
| `-i` or `--input`       | Sets the input file for encryption.                        |
| `-o` or `--output`      | Sets the output file for the encrypted payload (default is "output.png").|
| `-k` or `--key`         | Sets the key for payload encryption (default is "key").    |
| `-s` or `--suppress`    | Suppresses output messages.                                |
| `-f` or `--offset`      | Sets the offset (default is 10).                           |
| `-p` or `--payload`     | Sets the payload (default is "hello").                     |
| `-t` or `--type`        | Sets the type (default is "PNG").                          |
|                         |                                                           |
| **Decryption Options**  |                                                           |
| `-i` or `--input`       | Sets the input file for decryption.                        |
| `-o` or `--output`      | Sets the output file for the decrypted payload (default is "output.png").|
| `-k` or `--key`         | Sets the key for payload decryption (default is "key").    |
| `-s` or `--suppress`    | Suppresses output messages.                                |
| `-f` or `--offset`      | Sets the offset (default is 10).                           |
| `-p` or `--payload`     | Sets the payload (default is "hello").                     |
| `-t` or `--type`        | Sets the type (default is "PNG").                          |
|                         |                                                           |
| **Metadata Options**    |                                                           |
| `-i` or `--input`       | Sets the input image file for metadata extraction.         |
| `-n` or `--nb-chunks`   | Read a specific number of chunks (default is 100).          |
| `-s` or `--start`       | Sets the index of the start chunk to read from (default 0). |
| `-e` or `--end`         | Sets the index of the end chunk to stop reading at (default 100).|
| `-r` or `--suppress`    | Suppresses output messages.                                |

## 🤝 Contributing

Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement, please engage with the project on [GitHub](https://github.com/wiseaidev/stegano).
Your contributions help improve this crate for the community.

## 📄 License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
