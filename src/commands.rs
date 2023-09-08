use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;
use std::fs;

pub fn encode(args: EncodeArgs) -> Result<()> {
    let bytes = fs::read(&args.path)?;
    let mut png: Png = bytes.as_slice().try_into()?;
    let chunk_type: ChunkType = args.chunk_type.parse()?;
    let chunk: Chunk = Chunk::new(chunk_type, args.message.clone().into_bytes());
    png.append_chunk(chunk);
    if let Some(output) = args.output {
        fs::write(output, png.as_bytes())?;
    } else {
        fs::write(&args.path, png.as_bytes())?;
    }
    println!(
        "Encoded \"{}\" into \"{}\"",
        args.message,
        args.path.display()
    );
    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    let bytes = fs::read(args.path)?;
    let png: Png = bytes.as_slice().try_into()?;
    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        println!("{}", chunk.data_as_string()?);
    } else {
        println!("Chunk type \"{}\" not found in png", &args.chunk_type);
    }
    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    let bytes = fs::read(&args.path)?;
    let mut png: Png = bytes.as_slice().try_into()?;
    png.remove_chunk(&args.chunk_type)?;
    fs::write(&args.path, &png.as_bytes())?;
    println!("Removed chunk type \"{}\" from png", args.chunk_type);
    Ok(())
}

pub fn print(args: PrintArgs) -> Result<()> {
    let bytes = fs::read(args.path)?;
    println!("{:?}", &bytes.as_slice());
    Ok(())
}
