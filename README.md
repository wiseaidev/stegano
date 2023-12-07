# 🕵️‍♂️ Stegano

[![Crates.io](https://img.shields.io/crates/v/stegano.svg)](https://crates.io/crates/stegano)
[![Crates.io Downloads](https://img.shields.io/crates/d/stegano)](https://crates.io/crates/stegano)
![Rust](https://img.shields.io/badge/rust-stable-orange)
[![License](https://img.shields.io/crates/l/stegano.svg)](https://opensource.org/licenses/MIT)

![Logo](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/ushyby2i2b14qbndshg5.png)

> 🚀 `stegano`: Stegano is a powerful and versatile steganography tool designed to empower you with a wide range of image manipulation and data encryption capabilities.

## 📖 Table of Contents

- [Installation](#-installation)
- [Features](#-features)
- [Usage](#-usage)
- [Options](#-options)
- [Contributing](#-contributing)
- [License](#-license)

## 🚀 Installation

To install `stegano`, use the following Cargo command:

```bash
cargo install --locked stegano
```

## 🖼️ Features

- Parse both PNG and JPEG images effortlessly.
- Gain insights into the internal structure of PNG images with a hex dump view.
- Seamlessly inject payloads into images at the `IEND - 8` bytes position marker.
- Safely hide your data without compromising the integrity of the original image.
- Utilize the AES-128 algorithm for secure encryption and decryption of hidden data.
- No limitations on the length of the payload that can be injected.

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

   ---- Header ----
   00000000 | 89 50 4E 47 0D 0A 1A 0A | .PNG....
   ----- End ------

   ---- Chunk #0 ----
   Offset: 8
   Size: 71
   CRC: 48142467
   00000008 | 00 00 00 0D 49 48 44 52 00 00 05 A0 00 00 03 D0 08 06 00 00 | ....IHDR............
   00000028 | 00 C0 52 DC 33 00 00 00 01 73 52 47 42 00 AE CE 1C E9 00 00 | ..R.3....sRGB.......
   00000048 | 20 00 49 44 41 54 78 5E EC 9D 07 98 24 45 D9 80 BF DD 0B 5C | ..IDATx^....$E.....\
   00000068 | E2 8E 8C 24 C9 41 50 09 82 A8 80 | ...$.AP....
   ------- End -------

   ---- Chunk #1 ----
   Offset: 83
   Size: 17
   CRC: 241924a7
   00000083 | A2 64 38 B2 81 20 19 91 24 0A 28 48 D2 9F AC 92 91 | .d8.....$.(H.....
   ------- End -------

   ---- Chunk #2 ----
   Offset: 112
   Size: 3
   CRC: 9e0e55d5
   00000112 | ED D9 EE | ...
   ------- End -------

   ---- Chunk #3 ----
   Offset: 127
   Size: 51
   CRC: 400002d6
   00000127 | B2 53 5D F5 D5 5B D5 7D C7 3B DF 7E D5 F5 C7 DB EF EF 15 AE | .S]..[.}.;.~........
   00000147 | 52 08 F4 F6 82 36 0A 16 1E A5 6C 34 3A 85 00 04 20 00 01 08 | R....6....l4:.......
   00000167 | 40 00 02 10 80 00 04 20 00 01 08 | @..........
   ------- End -------

   ---- Chunk #4 ----
   Offset: 190
   Size: 4
   CRC: fa7621a0
   00000190 | BA E0 51 D6 | ..Q.
   ------- End -------

   ---- Chunk #5 ----
   Offset: 206
   Size: 60
   CRC: 1084000
   00000206 | FD EF 30 7A 84 00 04 20 00 01 08 40 00 02 10 80 00 04 20 00 | ..0z.......@........
   00000226 | 01 08 40 00 02 3E 09 20 5F 03 9A 88 68 9F FB AA D6 17 02 DA | ..@..>.._...h.......
   00000246 | 33 53 E4 33 C2 D9 F3 96 A2 3B 08 40 00 02 10 80 00 04 20 00 | 3S.3.....;.@........
   ------- End -------

   ---- Chunk #6 ----
   Offset: 278
   Size: 2
   CRC: 5b4823a1
   00000278 | 02 03 | ..
   ------- End -------

   ---- Chunk #7 ----
   Offset: 292
   Size: 4
   CRC: 3d6e1dba
   00000292 | 4B 3E 23 9A | K>#.
   ------- End -------

   ---- Chunk #8 ----
   Offset: 308
   Size: 4
   CRC: 42000
   00000308 | 00 02 10 80 | ....
   ------- End -------

   ---- Chunk #9 ----
   Offset: 324
   Size: 1
   CRC: 886924b4
   00000324 | E0 | .
   ------- End -------
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

   ---- Header ----
   00000000 | 89 50 4E 47 0D 0A 1A 0A | .PNG....
   ----- End ------

   ---- Chunk #0 ----
   Offset: 8
   Size: 71
   CRC: 48142467
   00000008 | 00 00 00 0D 49 48 44 52 00 00 05 A0 00 00 03 D0 08 06 00 00 | ....IHDR............
   00000028 | 00 C0 52 DC 33 00 00 00 01 73 52 47 42 00 AE CE 1C E9 00 00 | ..R.3....sRGB.......
   00000048 | 20 00 49 44 41 54 78 5E EC 9D 07 98 24 45 D9 80 BF DD 0B 5C | ..IDATx^....$E.....\
   00000068 | E2 8E 8C 24 C9 41 50 09 82 A8 80 | ...$.AP....
   ------- End -------

   # Read 3 chunks starting from position 40000
   $ stegano show-meta -i image_file_name -s 40000 -e 200000 -n 3
   It is a valid PNG file. Let's process it! 

   ---- Header ----
   00000000 | 89 50 4E 47 0D 0A 1A 0A | .PNG....
   ----- End ------

   ---- Chunk #40000 ----
   Offset: 40000
   Size: 49
   CRC: e3a1975e
   00040000 | 16 03 F2 3B 23 05 BD 79 7E AE 37 2D 90 1C E8 B8 4F E7 2E 34 | ...;#..y~.7-....O..4
   00040020 | 42 9E 43 55 01 1D 6E 01 6D AD 7E 76 5A 40 CB 6A 6A 6B 2C 87 | B.CU..n.m.~vZ@.jjk,.
   00040040 | 87 06 84 5C 1D 8B 60 67 11 | ...\..`g.
   ------- End -------

   ---- Chunk #40001 ----
   Offset: 40061
   Size: 28
   CRC: 2200002
   00040061 | E3 40 40 07 F1 F6 C2 A1 20 00 02 20 00 02 20 00 02 20 00 02 | .@@.................
   00040081 | 20 00 02 20 00 02 20 00 | ........
   ------- End -------

   ---- Chunk #40002 ----
   Offset: 40101
   Size: 17
   CRC: 4f13b5a6
   00040101 | 11 B1 EC B9 11 A1 9E 89 AC 4A 58 4B D6 32 DB C7 D8 | .........JXK.2...
   ------- End -------
   ```

1. Encrypt and inject data in an image:

> [!NOTE]
Stegano automatically identifies the location of the `IEND` marker in a PNG image and injects the payload at a position 8 bytes before the IEND marker by default. This is done to prevent any corruption of the original image.

   ```bash
   $ stegano encrypt -i input_image_file_name -k 'pass' -p 'hello' -o output_image_file_name
   It is a valid PNG file. Let's process it! 

   ---- Header ----
   00000000 | 89 50 4E 47 0D 0A 1A 0A | .PNG....
   ----- End ------

   ------- Chunk -------
   Offset: 159028
   Size: 48
   CRC: 13a29bcc
   00159028 | 03 AE 73 9D 60 28 1A F1 20 A0 EB 10 39 11 28 9D FC 85 5E DB | ..s.`(......9.(...^.
   00159048 | D2 D3 7A 45 B1 71 EE 4F 4C 66 0C E8 FC 85 5E DB D2 D3 7A 45 | ..zE.q.OLf....^...zE
   00159068 | B1 71 EE 4F 4C 66 0C E8 | .q.OLf..
   -------- End --------

   Your payload has been encrypted and written at offset 159028 successfully!
   ```

> [!NOTE]
By default, the maximum key length is restricted to 16 characters.

1. Using the previously obtained offset, you can decrypt and extract the secret information from an image, as well as remove the secret content from the image:

   ```bash
   $ stegano decrypt -i input_image_file_name -k 'pass' -f 159028 -o output_image_file_name -s
   Your decrypted secret is: "hello"
   ```

1. if wrong key is provided: 

   ```bash
   $ stegano decrypt -i input_image_file_name -k 'invalid' -f 159028 -o output_image_file_name
   Your decrypted secret is: "qji~s"
   ```

1. if wrong key and wrong offset are provided: 

   ```bash
   $ stegano decrypt -i input_image_file_name -k 'invalid' -f 159024 -o output_image_file_name
   It is a valid PNG file. Let's process it! 

   ---- Header ----
   00000000 | 89 50 4E 47 0D 0A 1A 0A | .PNG....
   ----- End ------

   ------- Chunk -------
   Offset: 159021
   Size: 36
   CRC: 1348f762
   00159024 | F1 B0 E8 48 9E AD 1E 56 A5 82 7C 3B 14 D4 8C 1D E7 1A 90 47 | ...H...V..|;.......G
   00159044 | A0 1D D2 90 04 71 2F 01 AA 91 86 6D | .....q/....m
   -------- End --------

   Your decrypted secret is: "��H��\u{1e}V��|;\u{14}Ԍ\u{1d}�\u{1a}�G�\u{1d}Ґ\u{4}q/\u{1}���m"
   ```

1. if correct key and wrong offset are provided: 

   ```bash
   $ stegano decrypt -i input_image_file_name -k 'pass' -f 159024 -o output_image_file_name
   It is a valid PNG file. Let's process it! 

   ---- Header ----
   00000000 | 89 50 4E 47 0D 0A 1A 0A | .PNG....
   ----- End ------

   ------- Chunk -------
   Offset: 159021
   Size: 36
   CRC: fc855edb
   00159024 | 67 47 07 E6 DC 5D 27 34 CB 51 A4 63 66 C1 C5 F4 5A A2 6B 5C | gG...]'4.Q.cf...Z.k\
   00159044 | 28 24 BF 53 BC 21 59 04 1B F6 FC 21 | ($.S.!Y....!
   -------- End --------

   Your decrypted secret is: "gG\u{7}��]'4�Q�cf���Z�k\\($�S�!Y\u{4}\u{1b}��!"
   ```

> [!NOTE]
By default, Stegano uses the AES-128 encryption algorithm. The maximum key length supported is 16 characters. If you provide a longer key, it will be automatically padded or truncated to fit the required length.

## 🎨 Options

| Option                  | Description                                               |
|-------------------------|-----------------------------------------------------------|
| **Encryption Options**  |                                                           |
| `-a` or `--algo`        | Sets the algorithm for encryption (default is "aes").        |
| `-i` or `--input`       | Sets the input file for encryption.                        |
| `-o` or `--output`      | Sets the output file for the encrypted payload (default is "output.png").|
| `-k` or `--key`         | Sets the key for payload encryption (default is "key").    |
| `-s` or `--suppress`    | Suppresses output messages.                                |
| `-f` or `--offset`      | Sets the offset (default is 9999999999 for auto injection: IEND - 8 bytes).  |
| `-p` or `--payload`     | Sets the payload (default is "hello").                     |
| `-t` or `--type`        | Sets the type (default is "PNG").                          |
|                         |                                                           |
| **Decryption Options**  |                                                           |
| `-a` or `--algo`        | Sets the algorithm for decryption (default is "aes").        |
| `-i` or `--input`       | Sets the input file for decryption.                        |
| `-o` or `--output`      | Sets the output file for the decrypted payload (default is "output.png").|
| `-k` or `--key`         | Sets the key for payload decryption (default is "key").    |
| `-s` or `--suppress`    | Suppresses output messages.                                |
| `-f` or `--offset`      | Sets the offset (default is 9999999999) for auto decryption: IEND - 8 bytes).  |
| `-p` or `--payload`     | Sets the payload (default is "hello").                     |
| `-t` or `--type`        | Sets the type (default is "PNG").                          |
|                         |                                                           |
| **Metadata Options**    |                                                           |
| `-i` or `--input`       | Sets the input image file for metadata extraction.         |
| `-n` or `--nb-chunks`   | Read a specific number of chunks (default is 100).          |
| `-s` or `--start`       | Sets the index of the start chunk to read from (default 0). |
| `-e` or `--end`         | Sets the index of the end chunk to stop reading at (default 100).|
| `-r` or `--suppress`    | Suppresses output messages.                                |
| `-z` or `--read-end`    | Read from start or end of file (default is reading from the start of image). |

## 🤝 Contributing

Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement, please engage with the project on [GitHub](https://github.com/wiseaidev/stegano).
Your contributions help improve this crate for the community.

## 📄 License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
