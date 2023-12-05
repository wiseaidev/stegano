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

   ---- Header ----
   00000000 | 89 50 4E 47 0D 0A 1A 0A | .PNG....
   ---- End ----

   ---- Chunk #0 ----
   Offset: 13
   Size: 68
   CRC: fa130394
   00000013 | 00 0A 36 08 06 00 00 00 AF 77 9F 68 00 00 00 01 73 52 47 42 | ..6......w.h....sRGB
   00000033 | 00 AE CE 1C E9 00 00 20 00 49 44 41 54 78 5E EC DD 75 BC 6F | .........IDATx^..u.o
   00000053 | 55 9D FF F1 CF 39 B7 83 1B 70 69 91 54 50 50 46 09 51 41 C5 | U....9...pi.TPPF.QA.
   00000073 | 8E F9 19 94 98 E0 08 12 | ........
   ------ End ------

   ---- Chunk #1 ----
   Offset: 98
   Size: 70
   CRC: 80000208
   00000098 | 42 74 08 09 0B C5 E2 27 17 A5 E3 22 48 DE E4 C6 39 BF C7 DA | Bt.....'..."H...9...
   00000118 | DF DA B1 F6 5E B9 BF 75 5E E7 9F E1 9E EF CA E7 5E FB 7B F9 | ....^..u^.......^.{.
   00000138 | F1 7B FB 59 23 97 5E 7D E3 B8 F0 13 4D 60 7C 1C CE 68 98 0C | .{.Y#.^}....M`|..h..
   00000158 | 84 00 02 08 20 80 00 02 08 20 | ..........
   ------ End ------

   ---- Chunk #2 ----
   Offset: 185
   Size: 2
   CRC: 20800002
   00000185 | 02 08 | ..
   ------ End ------

   ---- Chunk #3 ----
   Offset: 204
   Size: 8
   CRC: 460800c6
   00000204 | 32 32 32 54 FB E9 F5 66 | 222T...f
   ------ End ------

   ---- Chunk #4 ----
   Offset: 229
   Size: 2
   CRC: 20800002
   00000229 | 02 08 | ..
   ------ End ------

   ---- Chunk #5 ----
   Offset: 248
   Size: 8
   CRC: 20800002
   00000248 | 00 02 08 20 80 00 02 08 | ........
   ------ End ------

   ---- Chunk #6 ----
   Offset: 273
   Size: 8
   CRC: 20190601
   00000273 | 95 00 60 24 4B C2 7F 91 | ..`$K...
   ------ End ------

   ---- Chunk #7 ----
   Offset: 298
   Size: 4
   CRC: 40000104
   00000298 | 00 01 04 10 | ....
   ------ End ------

   ---- Chunk #8 ----
   Offset: 319
   Size: 1
   CRC: 4104000
   00000319 | 01 | .
   ------ End ------

   ---- Chunk #9 ----
   Offset: 337
   Size: 16
   CRC: 20820
   00000337 | 23 E1 BF 08 88 A6 21 B8 5A D9 24 C4 E7 08 20 80 | #.....!.Z.$.....
   ------ End ------
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
   ---- End ----

   ---- Chunk #0 ----
   Offset: 13
   Size: 68
   CRC: fa130394
   00000013 | 00 0A 36 08 06 00 00 00 AF 77 9F 68 00 00 00 01 73 52 47 42 | ..6......w.h....sRGB
   00000033 | 00 AE CE 1C E9 00 00 20 00 49 44 41 54 78 5E EC DD 75 BC 6F | .........IDATx^..u.o
   00000053 | 55 9D FF F1 CF 39 B7 83 1B 70 69 91 54 50 50 46 09 51 41 C5 | U....9...pi.TPPF.QA.
   00000073 | 8E F9 19 94 98 E0 08 12 | ........
   ------ End ------

   # Read 3 chunks starting from position 10000
   $ stegano show-meta -i image_file_name -s 10000 -e 200000 -n 3
   It is a valid PNG file. Let's process it! 

   ---- Header ----
   00000000 | 89 50 4E 47 0D 0A 1A 0A | .PNG....
   ---- End ----

   ---- Chunk #10000 ----
   Offset: 13
   Size: 68
   CRC: fa130394
   00000013 | 00 0A 36 08 06 00 00 00 AF 77 9F 68 00 00 00 01 73 52 47 42 | ..6......w.h....sRGB
   00000033 | 00 AE CE 1C E9 00 00 20 00 49 44 41 54 78 5E EC DD 75 BC 6F | .........IDATx^..u.o
   00000053 | 55 9D FF F1 CF 39 B7 83 1B 70 69 91 54 50 50 46 09 51 41 C5 | U....9...pi.TPPF.QA.
   00000073 | 8E F9 19 94 98 E0 08 12 | ........
   ------ End ------

   ---- Chunk #10001 ----
   Offset: 98
   Size: 70
   CRC: 80000208
   00000098 | 42 74 08 09 0B C5 E2 27 17 A5 E3 22 48 DE E4 C6 39 BF C7 DA | Bt.....'..."H...9...
   00000118 | DF DA B1 F6 5E B9 BF 75 5E E7 9F E1 9E EF CA E7 5E FB 7B F9 | ....^..u^.......^.{.
   00000138 | F1 7B FB 59 23 97 5E 7D E3 B8 F0 13 4D 60 7C 1C CE 68 98 0C | .{.Y#.^}....M`|..h..
   00000158 | 84 00 02 08 20 80 00 02 08 20 | ..........
   ------ End ------

   ---- Chunk #10002 ----
   Offset: 185
   Size: 2
   CRC: 20800002
   00000185 | 02 08 | ..
   ------ End ------
   ```

1. Encrypt an inject data in an image: 

   ```bash
   $ stegano encrypt -i input_image_file_name -k 'pass' -p 'hello' -f 159028 -o output_image_file_name -s
   Image encoded and written successfully!
   ```

1. Decrypt, extract secret from an image and remove the secret from the image: 

   ```bash
   $ stegano decrypt -i input_image_file_name -k 'pass' -f 159028 -o output_image_file_name -s
   Your decoded secret is: "hello"
   ```

1. if wrong key is provided: 

   ```bash
   $ stegano decrypt -i input_image_file_name -k 'invalid' -f 159028 -o output_image_file_name
   Your decoded secret is: "qji~s"
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
   00159021 | 69 6E 76 61 74 6D 7B 76 71 1B 4D C0 1F 8D 24 7F 7C 61 FC 97 | invatm{vq.M...$.|a..
   00159041 | 19 9A B0 0D 99 41 D5 E5 BD 20 C0 79 B2 4A 7B C5 | .....A.....y.J{.
   -------- End --------

   Your decoded secret is: "invatm{vq\u{1b}M�\u{1f}�$\u{7f}|a��\u{19}��\r�A�� �y�J{�"
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
   CRC: 1348f762
   00159021 | 70 61 73 73 68 65 6C 6C 6F 0C 5F DF 06 88 3E 62 7A 61 E3 8D | passhello._...>bza..
   00159041 | 0D 92 AD 08 88 4C CF F2 A4 2F C5 6B AE 42 6C DF | .....L.../.k.Bl.
   -------- End --------

   Your decoded secret is: "passhello\u{c}_�\u{6}�>bza�\r��\u{8}�L��/�k�Bl�"
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
