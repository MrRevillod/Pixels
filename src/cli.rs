
use crate::Args;
use crate::utils::*;
use crate::types::*;

use webp::*;

pub fn run(args: &Args) -> CliResult<()> { 

    let img_path = get_path_from_input(&args.input)?;
    
    validate_mime_type(&img_path)?;

    let renamed = get_path_from_rename(&img_path, &args.rename);

    let quality = normalize_quality(args.quality)?;

    let img = image::open(&img_path).unwrap();

    let encoder: Encoder = Encoder::from_image(&img).unwrap();
    let webp: WebPMemory = encoder.encode(quality as f32);
    let output_path = renamed.with_extension("webp");
    
    std::fs::write(&output_path, &*webp).unwrap();

    Ok(())
}
