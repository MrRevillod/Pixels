
use std::fs;
use std::path::PathBuf;

use webp::*;

use crate::Args;
use crate::utils::*;
use crate::types::*;

pub fn run(args: &Args) -> CliResult<()> { 

    let input_path = get_path_from_input(&args.input)?;
    
    validate_mime_type(&input_path)?;

    let output_path = get_output_path(&input_path, &args.rename);

    let quality = normalize_quality(args.quality)?;
    let img = image::open(&input_path).unwrap();

    let encoder: Encoder = Encoder::from_image(&img).unwrap();
    let webp: WebPMemory = encoder.encode(quality as f32);
    let output_path = output_path.with_extension("webp");
    
    fs::write(&output_path, &*webp).unwrap();
    stats(&input_path, &output_path);

    Ok(())
}

fn stats(input: &PathBuf, output: &PathBuf) {

    let input_size = fs::metadata(input).unwrap().len() as i64 / 1024;
    let output_size = fs::metadata(output).unwrap().len() as i64 / 1024;

    let diff = input_size - output_size;

    println!("");
    println!("{} Initial image size: {}Kb", *TERM_INFO, input_size);
    println!("{} Final image size: {}Kb", *TERM_INFO, output_size);
    println!("{} Compressed: {}Kb", *TERM_INFO, diff);

    println!("{} Image saved at: {:?}", *TERM_INFO, output);

    println!("");
    println!("{} Done!", *TERM_OK);

}