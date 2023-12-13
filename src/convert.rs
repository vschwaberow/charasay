use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn convert_png_to_chara(
    png_path: &str,
    chara_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Converting {} to {}", png_path, chara_path.display());
    // Open the PNG image and resize it to a thumbnail of size 80x50
    let img = image::open(png_path)?.thumbnail(80, 50);
    // Convert the image to grayscale
    let img = img.to_luma8();

    // Define the ASCII characters used for the ASCII art
    let ascii_chars = " .:-=+*#%@";
    let mut ascii_art = String::new();

    // Iterate over each pixel in the image
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let brightness = pixel[0] as usize;
            // Map the brightness value to an ASCII character index
            let ascii_index = brightness * ascii_chars.len() / 256;
            // Append the corresponding ASCII character to the ASCII art string
            ascii_art.push(ascii_chars.chars().nth(ascii_index).unwrap());
        }
        // Add a newline character at the end of each row
        ascii_art.push('\n');
    }

    // take the filename of the png, erase the extension and add .chara
    let chara_path = chara_path.with_extension("chara");

    // Create a new file and write the ASCII art to it
    let mut file = File::create(chara_path)?;
    file.write_all(ascii_art.as_bytes())?;

    Ok(())
}
