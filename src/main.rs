use image::{io::Reader as ImageReader, GenericImageView};
use clap::Parser;

// A simple program to generate from an image an ASCII art
#[derive(Parser, Debug)]
struct Args {
    // Scale image down
    #[arg(long, default_value_t = 1)]
    scale: u32,

    // Image path
    #[arg(long)]
    path: String,

    // Characters mapper
    #[arg(short)]
    chars: Option<String>,
}

fn main() -> Result<(), image::ImageError> {
    let args = Args::parse();
    let path = args.path;
    let scale = args.scale;
    let vecs: Vec<char> = args.chars.unwrap_or(" _･,-=+:;!?$W#@N".to_string()).chars().collect();
    let map_dom = 255_usize.saturating_div(vecs.len() - 2);

    let img = ImageReader::open(path)?.decode()?;
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            if y % (2 * scale) == 0 && x % scale == 0 {
                let pixel = img.get_pixel(x, y);
                let brightness = pixel[3].saturating_div(128).saturating_mul(
                    pixel[0]
                        .saturating_div(3)
                        .saturating_add(pixel[1].saturating_div(3))
                        .saturating_add(pixel[2].saturating_div(3)),
                );
                let index =
                    (brightness as usize).saturating_div(map_dom);
                print!("{}", vecs[index])
            }
        }
        if y % (2 * scale) == 0 {
            println!()
        }
    }
    Ok(())
}
