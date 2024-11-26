use bitcode::{Decode, Encode};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::{self, File};
use std::io::{BufWriter, Write};

#[derive(Encode, Decode, Debug)]
struct Frame {
    content: String,
}

fn encode_text_and_compress_frames_to_bitcode(
    input_dir: &str,
    output_file: &str,
) -> std::io::Result<()> {
    let mut files: Vec<_> = fs::read_dir(input_dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .collect();

    files.sort_by_key(|entry| entry.path());

    let file = File::create(output_file)?;
    let gz_encoder = GzEncoder::new(file, Compression::default());
    let mut writer = BufWriter::new(gz_encoder);

    for entry in files.iter() {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy();

        // Read file content
        let content = fs::read_to_string(&path)?;
        println!("Processing file: {}", file_name);

        // Create a Frame
        let frame = Frame { content };

        // Encode the frame
        let encoded: Vec<u8> = bitcode::encode(&frame);

        // Ensure that the encoded frame is of the expected size
        assert_eq!(encoded.len(), 3496, "Encoded frame size mismatch");

        // Write the encoded frame directly
        writer.write_all(&encoded)?;
    }

    writer.flush()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let input_dir = "./input";
    let output_file = "badapple.bitcode";
    encode_text_and_compress_frames_to_bitcode(input_dir, output_file)?;
    Ok(())
}
