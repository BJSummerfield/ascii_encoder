mod ascii;
mod error;

use ascii::Parser;
use error::Result;

fn main() -> Result<()> {
    let frames_directory = "./input/frames_thor_ascii";

    let output_file = "thor.delta.bitcode";
    let width = 172;
    let mut parser = Parser::new(frames_directory, output_file, width)?;
    parser.convert_frames_to_bitcode()?;
    Ok(())
}
