use clap::Parser;
use crc32_v2::byfour::crc32_little;
use std::fs::File;
use std::io::Write;
use stegano::cli::{Cli, SteganoCommands};
use stegano::jpeg::utils::read_jpeg_headers;
use stegano::models::MetaChunk;
use stegano::utils::xor_encode_decode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // Run the CLI.
    match args.command {
        Some(command) => match command {
            SteganoCommands::Encrypt(encrypt_cmd) => {
                let mut file = File::open(encrypt_cmd.input.clone())?;

                let mut meta_chunk = MetaChunk::new(&mut file, encrypt_cmd.suppress)
                    .expect("Error processing the png file!");

                let mut file_writer = File::create(encrypt_cmd.output.clone())?;
                let encoded_data =
                    xor_encode_decode(encrypt_cmd.payload.as_bytes(), &encrypt_cmd.key);
                if !encrypt_cmd.suppress {
                    println!("Encoded bytes {:?}", encoded_data);
                }
                // Calculate CRC for the encoded data
                let mut bytes_msb = Vec::new();
                bytes_msb
                    .write_all(&meta_chunk.chk.r#type.to_be_bytes())
                    .unwrap();
                bytes_msb.write_all(&encoded_data).unwrap();
                let crc = crc32_little(meta_chunk.chk.crc, &bytes_msb);

                // Update the MetaChunk with the encoded data and CRC
                meta_chunk.chk.data = encoded_data;
                meta_chunk.chk.crc = crc;

                // Create a new mutable reference to file_reader
                let mut file_reader = &file;

                meta_chunk.write_encrypted_data(&mut file_reader, &encrypt_cmd, &mut file_writer);

                println!("Image encoded and written successfully!");
            }
            SteganoCommands::Decrypt(decrypt_cmd) => {
                let mut file = File::open(decrypt_cmd.input.clone())?;

                let mut meta_chunk = MetaChunk::new(&mut file, decrypt_cmd.suppress)
                    .expect("Error processing the png file!");

                let mut file_writer = File::create(decrypt_cmd.output.clone()).unwrap();
                let mut file_reader = &file;
                meta_chunk.write_decrypted_data(&mut file_reader, &decrypt_cmd, &mut file_writer);
            }
            SteganoCommands::ShowMeta(show_meta_cmd) => {
                if show_meta_cmd.r#type.to_lowercase() == "jpeg" {
                    let _ = read_jpeg_headers(
                        &show_meta_cmd.input.clone(),
                        show_meta_cmd.start_chunk,
                        show_meta_cmd.end_chunk,
                        show_meta_cmd.nb_chunks,
                    );
                } else if show_meta_cmd.r#type.to_lowercase() == "png" {
                    let mut file = File::open(show_meta_cmd.input.clone())?;
                    let mut meta_chunk = MetaChunk::new(&mut file, show_meta_cmd.suppress)
                        .expect("Error processing the png file!");
                    meta_chunk.process_image(&mut file, &show_meta_cmd);
                }
                return Ok(());
            }
        },
        None => println!("\x1b[1;91mUnknown command. Use 'help' for usage instructions.\x1b[0m"),
    }
    Ok(())
}
