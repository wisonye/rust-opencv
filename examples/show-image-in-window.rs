use opencv::{core, highgui, imgcodecs, prelude::*};
use std::env;

/// For all available `Matrix`, plz have a look at:
/// https://docs.rs/opencv/0.46.3/opencv/core/prelude/trait.MatTrait.html
fn show_image_info(matrix: &core::Mat) {
    let mut messages = String::new();
    messages.push_str("\n[ Image Info ]:");

    let size_result = matrix.size();
    let size = size_result.as_ref().unwrap();
    messages.push_str(format!("\nresulotion: {} x {}", size.width, size.height).as_str());

    let channels_result = matrix.channels();
    let channels = channels_result.as_ref().unwrap();
    messages.push_str(&format!("\nIs grayscale: {}", *channels == 1));

    messages.push_str(&format!("\nDimension: {}", matrix.dims()));
    println!("{}", messages);
}

///
fn show_image_from_file(file_name: &str, image_read_mode: i32) {
    // Setup render window
    let window_name = "Image Preview";

    // `imread` (Image Read) return a `n-demensional Matrix` which contains all image pixels
    // and extra data
    let image = imgcodecs::imread(file_name, image_read_mode).unwrap();
    println!("image: {:#?}", &image);

    if image.empty().unwrap() {
        println!("\nImage load failed: {}\n", file_name);
        return;
    }

    show_image_info(&image);

    // highgui::named_window(window_name, highgui::WindowFlags::WINDOW_AUTOSIZE as i32).unwrap();

    // WINDOW_AUTOSIZE     - the user cannot resize the window, the size is constrainted by the image displayed.
    // WINDOW_FREERATIO    - the image expends as much as it can (no ratio constraint).
    // WINDOW_FULLSCREEN   - change the window to fullscreen.
    // WINDOW_GUI_EXPANDED - status bar and tool bar
    // WINDOW_GUI_NORMAL   - old fashious way
    // WINDOW_KEEPRATIO    - the ratio of the image is respected.
    // WINDOW_NORMAL       - the user can resize the window (no constraint) / also use to switch a fullscreen window to a normal size.
    // WINDOW_OPENGL       - window with opengl support.
    let window_flags = highgui::WINDOW_AUTOSIZE
        | highgui::WINDOW_KEEPRATIO
        | highgui::WINDOW_OPENGL
        | highgui::WINDOW_GUI_NORMAL;
    highgui::named_window(window_name, window_flags).unwrap();

    highgui::imshow(window_name, &image).unwrap();
    highgui::wait_key(10000).unwrap();
}

///
fn main() {
    let image_file_name = env::args().nth(1);
    if let None = image_file_name {
        println!("\nPlease provide an image file name:)\n");
        return;
    }

    // imgcodecs::IMREAD_UNCHANGED           - return the loaded image as is (with alpha channel, otherwise it gets cropped). Ignore EXIF orientation.
    // imgcodecs::IMREAD_GRAYSCALE           - Always convert image to the single channel grayscale image (codec internal conversion).
    // imgcodecs::IMREAD_COLOR               - Always convert image to the 3 channel BGR color image.
    // imgcodecs::IMREAD_ANYDEPTH            - Return 16-bit/32-bit image when the input has the corresponding depth, otherwise convert it to 8-bit.
    // imgcodecs::IMREAD_ANYCOLOR            - The image is read in any possible color format.
    // imgcodecs::IMREAD_LOAD_GDAL           - Use the gdal driver for loading the image.
    // imgcodecs::IMREAD_REDUCED_GRAYSCALE_2 - Always convert image to the single channel grayscale image and the image size reduced 1/2.
    // imgcodecs::IMREAD_REDUCED_COLOR_2     - Always convert image to the 3 channel BGR color image and the image size reduced 1/2.
    // imgcodecs::IMREAD_REDUCED_GRAYSCALE_4 - Always convert image to the single channel grayscale image and the image size reduced 1/4.
    // imgcodecs::IMREAD_REDUCED_COLOR_4     - Always convert image to the 3 channel BGR color image and the image size reduced 1/4.
    // imgcodecs::IMREAD_REDUCED_GRAYSCALE_8 - Always convert image to the single channel grayscale image and the image size reduced 1/8.
    // imgcodecs::IMREAD_REDUCED_COLOR_8     - Always convert image to the 3 channel BGR color image and the image size reduced 1/8.
    // imgcodecs::IMREAD_IGNORE_ORIENTATION  - Do not rotate the image according to EXIF's orientation flag.
    let read_mode = imgcodecs::IMREAD_COLOR;
    // let read_mode = imgcodecs::IMREAD_GRAYSCALE;

    show_image_from_file(image_file_name.unwrap().as_str(), read_mode);
}
