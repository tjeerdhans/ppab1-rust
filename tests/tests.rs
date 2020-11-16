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
