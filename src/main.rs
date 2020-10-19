mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use structopt::StructOpt;
use anyhow::{Result};

use args::PngMeArgs;
use commands::{encode, decode, remove, print};


fn main() -> Result<()> {
    let args = PngMeArgs::from_args();
    println!("{:?}", args);
    match args {
        PngMeArgs::Encode(encode_args) => encode(encode_args),
        PngMeArgs::Decode(decode_args) => decode(decode_args),
        PngMeArgs::Remove(remove_args) => remove(remove_args),
        PngMeArgs::Print(print_args) => print(print_args),
    }
}
