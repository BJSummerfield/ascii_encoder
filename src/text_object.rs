use crate::error::Result;
use bitcode::{Decode, Encode};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::{
    fs::{self, DirEntry, File},
    io::{BufWriter, Write},
};

#[derive(Encode, Decode, Debug)]
struct Frame {
    content: String,
}

pub struct TextObject {
    frame_resolution: usize,
    frames_directory: Vec<DirEntry>,
    writer: BufWriter<GzEncoder<File>>,
}

impl TextObject {
    pub fn new(
        frame_resolution: usize,
        frames_directory: &str,
        output_file: &str,
    ) -> Result<TextObject> {
        let frames_directory = Self::collect_files(frames_directory)?;
        let output_file = File::create(format!("./output/{}.gzip", output_file))?;

        let gz_encoder = GzEncoder::new(output_file, Compression::default());
        let writer = BufWriter::new(gz_encoder);

        let text_object = TextObject {
            frame_resolution,
            frames_directory,
            writer,
        };
        Ok(text_object)
    }

    fn collect_files(frames_directory: &str) -> Result<Vec<DirEntry>> {
        let mut files: Vec<DirEntry> = fs::read_dir(frames_directory)?
            .filter_map(std::io::Result::ok)
            .filter(|entry| entry.path().is_file())
            .collect();

        files.sort_by_key(|entry| entry.path());

        Ok(files)
    }

    pub fn convert_frames_to_bitcode(&mut self) -> Result<()> {
        for file in self.frames_directory.iter() {
            let path = file.path();
            let file_name = path.file_name().unwrap().to_string_lossy();

            let content = fs::read_to_string(&path)?;
            println!("Processing file: {}", file_name);
            let frame = Frame { content };
            let encoded: Vec<u8> = bitcode::encode(&frame);

            assert_eq!(
                encoded.len(),
                self.frame_resolution,
                "Encoded frame size mismatch"
            );
            self.writer.write_all(&encoded)?;
        }
        self.writer.flush()?;
        Ok(())
    }
}
