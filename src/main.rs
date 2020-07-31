extern crate image;
use image::DynamicImage;
use image::GenericImageView;

fn main() {
    println!("asciiart");
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = load_image("tests/images/ascii-pineapple.jpg");

    println!("{}", get_dimensions(img));
}

fn load_image(filename: &str) -> DynamicImage {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    image::open(filename).unwrap()
}

fn get_dimensions(img: DynamicImage) -> String {
    // The dimensions method returns the images width and height.
    format!(
        "Image size: {} x {}",
        img.dimensions().0,
        img.dimensions().1
    )
}
#[derive(Debug, Eq)]
struct Rgb(u8, u8, u8);

impl PartialEq for Rgb {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

fn get_image_matrix(img: DynamicImage) -> Vec<Vec<Rgb>> {
    let (width, height) = img.dimensions();
    let mut matrix = Vec::with_capacity(height as usize);
    for y in 0..height {
        let mut row = Vec::with_capacity(width as usize);
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            row.push(Rgb(pixel[0], pixel[1], pixel[2]));
        }
        matrix.push(row);
    }

    matrix
}

fn get_brightness_matrix(rgb_matrix: Vec<Vec<Rgb>>) -> Vec<Vec<u8>> {
    let mut matrix = Vec::with_capacity(rgb_matrix.len());
    for row in rgb_matrix.iter() {
        let mut brightness_row = Vec::with_capacity(row.len());
        for rgb in row.iter() {
            let b: u16 = (rgb.0 as u16 + rgb.1 as u16 + rgb.2 as u16) / 3;
            brightness_row.push(b as u8);
        }
        matrix.push(brightness_row)
    }

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_pineapple_image_and_show_dimensions() {
        // Use the open function to load an image from a Path.
        // `open` returns a `DynamicImage` on success.
        let img = load_image("tests/images/ascii-pineapple.jpg");
        let dims = get_dimensions(img);
        assert_eq!("Image size: 700 x 467", dims);
    }

    #[test]
    fn test_pixel_data_array() {
        let img = load_image("tests/images/ascii-pineapple.jpg");
        let matrix = get_image_matrix(img);

        assert_eq!(matrix[0][0], Rgb(1, 116, 209));
    }

    #[test]
    fn test_brightness_matrix() {
        let img = load_image("tests/images/ascii-pineapple.jpg");
        let matrix = get_image_matrix(img);
        let brightness_matrix = get_brightness_matrix(matrix);
        assert_eq!(brightness_matrix[0][0], 108);
    }
}
