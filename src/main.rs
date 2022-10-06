use image::io::Reader as ImageReader;
use image::{GenericImageView, ImageError, Rgba};
use rand::{seq::IteratorRandom, thread_rng};
use std::fmt::Display;
use std::io::Error as IOError;

/// Custom error type to handle all errors
#[derive(Debug)]
enum CliError {
    ImageError,
    ReadError(IOError),
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError::ReadError(error)
    }
}

impl From<ImageError> for CliError {
    fn from(_: ImageError) -> Self {
        CliError::ImageError
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CliError::ImageError => write!(f, "Cannot decode image"),
            CliError::ReadError(err) => write!(f, "{}", err),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Colors {
    red: u8,
    green: u8,
    blue: u8,
}

/// convert from rgba values to custom color enum
impl From<Rgba<u8>> for Colors {
    fn from(pixel: Rgba<u8>) -> Self {
        Colors {
            red: pixel[0],
            green: pixel[1],
            blue: pixel[2],
        }
    }
}

/// get distance from 2 colors on a 2d map using euclidean algorithm
fn euclidean_distance(color1: Rgba<u8>, color2: Rgba<u8>) -> i32 {
    unimplemented!()
}

fn kmeans(colors: &Vec<Rgba<u8>>, k: usize) {
    let mut rng = thread_rng();
    let sample = colors.into_iter().choose_multiple(&mut rng, k);
    println!("{:?}", sample);
}

/// Each pixels are of type rgba(red, green, blue, alpha)
fn generate_pixels_from_img(path: &str) -> Result<Vec<Rgba<u8>>, CliError> {
    let img = ImageReader::open(path)?.decode()?;
    let mut pixel_array: Vec<Rgba<u8>> = Vec::new();
    for pixel in img.pixels().map(|value| value.2) {
        pixel_array.push(pixel);
    }

    Ok(pixel_array)
}

fn main() -> Result<(), CliError> {
    let colors = generate_pixels_from_img("img.jpg")?;
    kmeans(&colors, 5);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_color_conversion() {
        let color: Rgba<u8> = Rgba([3, 4, 5, 255]);

        assert_eq!(
            Colors {
                red: 3,
                green: 4,
                blue: 5
            },
            Colors::from(color)
        );
    }
}
