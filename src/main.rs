mod error;
mod text_object;

use error::Result;
use text_object::TextObject;

fn main() -> Result<()> {
    let input_dir = "./input";
    let output_file = "badapple.bitcode";
    let frame_resolution = 3496;
    let mut text_object = TextObject::new(frame_resolution, input_dir, output_file)?;
    text_object.convert_frames_to_bitcode()?;
    Ok(())
}
