use crate::chunk_type::ChunkType;
use std::path::PathBuf;
use std::string::String;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pngme", about = "Hide secret messages in PNG files")]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "encode", about = "Encode a message into a PNG file")]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
    pub message: String,
    pub output_file: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "decode", about = "Decode a message stored in a PNG file")]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "remove", about = "Remove a message from a PNG file")]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "print",
    about = "Print a list of PNG chunks that can be searched for messages"
)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}
