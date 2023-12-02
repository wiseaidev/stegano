use clap::Parser;
use crc32_v2::byfour::crc32_little;
use std::fs::File;
use std::io::Write;
use stegano::cli::Cli;
use stegano::models::MetaChunk;
use stegano::utils::xor_encode_decode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let input = &args.input;
    let output = &args.output;
    let meta = &args.meta;
    let suppress = &args.suppress;
    let _offset = &args.offset;
    let _inject = &args.inject;
    let payload = &args.payload;
    // let r#type = &args.r#type;
    let encode = &args.encode;
    let decode = &args.decode;
    let key = &args.key;

    let mut file = File::open(input)?;

    let mut meta_chunk =
        MetaChunk::pre_process_image(&mut file, &args).expect("Error processing the png file!");

    if *meta {
        meta_chunk.process_image(&mut file, &args);
        return Ok(());
    } else if *encode {
        let mut file_writer = File::create(output)?;
        let encoded_data = xor_encode_decode(payload.as_bytes(), key);
        if !*suppress {
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

        meta_chunk.write_data(&mut file_reader, &args, &mut file_writer);

        println!("Image encoded and written successfully!");
    } else if *decode {
        let mut file_writer = File::create(output).unwrap();
        let mut file_reader = &file;
        meta_chunk.write_data(&mut file_reader, &args, &mut file_writer);
        // meta_chunk.process_image(&mut file);
    }

    Ok(())
}
