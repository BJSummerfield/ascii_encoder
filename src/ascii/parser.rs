use crate::{
    ascii::{Dictionary, Frame, ReferenceFrame},
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
    dictionary: Dictionary,
    frame_buffer: Vec<u8>, // Buffer to store frames temporarily
}

impl Parser {
    pub fn new(frames_directory: &str, output_file: &str) -> Result<Parser> {
        let regex = Regex::new(r"\x1b\[38;2;(\d{1,3});(\d{1,3});(\d{1,3})m(.)\x1b\[0m").unwrap();

        let frames_directory = Self::collect_files(frames_directory)?;
        let output_file = File::create(format!("./output/{}.gzip", output_file))?;

        let gz_encoder = GzEncoder::new(output_file, Compression::default());
        let writer = BufWriter::new(gz_encoder);

        let reference_frame = ReferenceFrame::new();
        let dictionary = Dictionary::new();

        Ok(Parser {
            frames_directory,
            writer,
            reference_frame,
            regex,
            dictionary,
            frame_buffer: Vec::new(), // Initialize empty frame buffer
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
        // Process each frame and write to temporary buffer
        for (file_index, file) in self.frames_directory.iter().enumerate() {
            let path = file.path();
            let file_name = path.file_name().unwrap().to_string_lossy();
            println!("Processing file: {}", file_name);
            let content = fs::read_to_string(&path)?;

            let mut frame_from_file = Frame::new();
            frame_from_file.capture_pixels(&content, &self.regex, &mut self.dictionary)?;

            let delta_frame = self.reference_frame.create_delta_frame(&frame_from_file);
            delta_frame.profile();
            self.reference_frame.apply_delta(&delta_frame);

            let encoded_frame: Vec<u8> = bitcode::encode(&delta_frame);

            // Write frame length and frame data to buffer
            let frame_length = (encoded_frame.len() as u32).to_le_bytes();
            self.frame_buffer.extend_from_slice(&frame_length);
            self.frame_buffer.extend_from_slice(&encoded_frame);

            println!(
                "File {} processed. Delta size: {} pixels.",
                file_index + 1,
                delta_frame.pixels.len()
            );
        }

        // Serialize dictionaries
        let encoded_chars = bitcode::encode(&self.dictionary.chars);
        let encoded_colors = bitcode::encode(&self.dictionary.colors);

        // Write dictionary lengths and data to the beginning
        let chars_length = (encoded_chars.len() as u32).to_le_bytes();
        let colors_length = (encoded_colors.len() as u32).to_le_bytes();
        println!("chars length: {}", encoded_chars.len());

        self.writer.write_all(&chars_length)?;
        self.writer.write_all(&encoded_chars)?;
        self.writer.write_all(&colors_length)?;
        self.writer.write_all(&encoded_colors)?;

        // Write frame buffer to the file
        self.writer.write_all(&self.frame_buffer)?;

        println!(
            "dictionary chars length: {}",
            self.dictionary.chars.chars.len()
        );
        println!(
            "dictionary colors length: {}",
            self.dictionary.colors.colors.len()
        );

        self.writer.flush()?;
        Ok(())
    }
}
