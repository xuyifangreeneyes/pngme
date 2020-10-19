use anyhow::{anyhow, Result};
use std::convert::AsRef;
use std::fs::File;
use std::io::prelude::*;
use std::str;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{ChunkType, Chunk, Png};

fn may_encode_message(chunk_type: &ChunkType) -> bool {
    chunk_type.is_valid() && !chunk_type.is_critical() && !chunk_type.is_public() && chunk_type.is_safe_to_copy()
}

pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file_path)?;
    if !may_encode_message(&args.chunk_type) {
        return Err(anyhow!("wrong chunk type for encoding message"));
    }
    let chunk = Chunk::new(args.chunk_type, args.message.into_bytes());
    let len = png.chunks().len();
    png.insert_chunk(len - 1, chunk);
    let bytes = png.as_bytes();
    let output_file = args.output_file.unwrap_or(args.file_path);
    let mut output_file = File::create(&output_file)?;
    output_file.write_all(bytes.as_ref())?;
    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::from_file(args.file_path)?;
    if let Some(chunk) = png.chunk_by_type(str::from_utf8(&args.chunk_type.bytes())?) {
        println!("secret message is \"{}\"", chunk.data_as_string()?);
        Ok(())
    } else {
        Err(anyhow!("cannot find any chunk of the type"))
    }
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file_path)?;
    if let Ok(chunk) = png.remove_chunk(str::from_utf8(&args.chunk_type.bytes())?) {
        println!("remove message: \"{}\"", chunk.data_as_string()?);
        let bytes = png.as_bytes();
        let mut output_file = File::create(&args.file_path)?;
        output_file.write_all(bytes.as_ref())?;
        Ok(())
    } else {
        Err(anyhow!("cannot find any chunk of the type"))
    }
}

pub fn print(args: PrintArgs) -> Result<()> {
    println!("chunk types of the chunks that can be searched for messages:");
    let png = Png::from_file(args.file_path)?;
    for chunk in png.chunks().iter() {
        let chunk_type = chunk.chunk_type();
        if may_encode_message(chunk_type) {
            println!("[{}]", str::from_utf8(&chunk_type.bytes())?);
        }
    }
    Ok(())
}
