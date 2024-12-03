use crate::{
    ascii::{Frame, ReferenceFrame},
    error::Result,
};
use flate2::{write::GzEncoder, Compression};
use regex::Regex;
use std::{
    fs::{self, DirEntry, File},
    io::{BufWriter, Write},
};

pub struct Parser {
    frames_directory: Vec<DirEntry>,
    writer: BufWriter<GzEncoder<File>>,
    reference_frame: ReferenceFrame,
    regex: Regex,
    ref_width: usize,
}

impl Parser {
    pub fn new(frames_directory: &str, output_file: &str, width: usize) -> Result<Parser> {
        let regex = Regex::new(r"\x1b\[38;2;(\d{1,3});(\d{1,3});(\d{1,3})m(.)\x1b\[0m").unwrap();

        let ref_width = width;
        let frames_directory = Self::collect_files(frames_directory)?;
        let output_file = File::create(format!("./output/{}.gzip", output_file))?;

        let gz_encoder = GzEncoder::new(output_file, Compression::default());
        let writer = BufWriter::new(gz_encoder);

        let reference_frame = ReferenceFrame::new();

        Ok(Parser {
            frames_directory,
            writer,
            reference_frame,
            regex,
            ref_width,
        })
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
        for (file_index, file) in self.frames_directory.iter().enumerate() {
            let path = file.path();
            let file_name = path.file_name().unwrap().to_string_lossy();
            println!("Processing file: {}", file_name);
            let content = fs::read_to_string(&path)?;

            let mut frame_from_file = Frame::new();
            frame_from_file.capture_pixels(&content, &self.regex, self.ref_width)?;

            let delta_frame = self.reference_frame.create_delta_frame(&frame_from_file);
            self.reference_frame.apply_delta(&delta_frame);

            let encoded_frame: Vec<u8> = bitcode::encode(&delta_frame);

            let frame_length = (encoded_frame.len() as u32).to_le_bytes();
            self.writer.write_all(&frame_length)?;
            self.writer.write_all(&encoded_frame)?;

            println!(
                "File {} processed. Delta size: {} pixels.",
                file_index + 1,
                delta_frame.pixels.len()
            );
        }

        self.writer.flush()?;
        Ok(())
    }
}
