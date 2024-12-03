mod ascii;
mod error;

use ascii::Parser;
use error::Result;

fn main() -> Result<()> {
    let frames_directory = "./input/frames_moana1_ascii";

    let output_file = "moana.charHash.bitcode";
    // let width = 210;
    let mut parser = Parser::new(frames_directory, output_file)?;
    parser.convert_frames_to_bitcode()?;
    Ok(())
}
