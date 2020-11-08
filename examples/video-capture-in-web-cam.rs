use opencv::{core, highgui, imgproc, prelude::*, videoio};

const TIPS: &'static str = "Press 'g' to toggle grayscale mode\nPress any key to exit";

///
fn draw_tips_on_frame(frame_image: &mut Mat) {
    let text_list = TIPS.split("\n");
    let text_coord = (5, 30); // X, Y
    let mut text_drawing_coord_y = text_coord.1;

    for (index, temp_text) in text_list.enumerate() {
        // Get the text size, then we can draw the text in the center
        // function variants that have the mask parameter.
        // FONT_HERSHEY_COMPLEX        - normal size serif font
        // FONT_HERSHEY_COMPLEX_SMALL  - smaller version of FONT_HERSHEY_COMPLEX
        // FONT_HERSHEY_DUPLEX         - normal size sans-serif font (more complex than FONT_HERSHEY_SIMPLEX)
        // FONT_HERSHEY_PLAIN          - small size sans-serif font
        // FONT_HERSHEY_SCRIPT_COMPLEX - more complex variant of FONT_HERSHEY_SCRIPT_SIMPLEX
        // FONT_HERSHEY_SCRIPT_SIMPLEX - hand-writing style font
        // FONT_HERSHEY_SIMPLEX        - normal size sans-serif font
        // FONT_HERSHEY_TRIPLEX        - normal size serif font (more complex than FONT_HERSHEY_COMPLEX)
        let font_scale = 0.8;
        let font_thickness = 1;
        let mut base_line = 0;
        let tip_size = imgproc::get_text_size(
            temp_text,
            imgproc::FONT_HERSHEY_DUPLEX,
            font_scale,
            font_thickness,
            &mut base_line,
        )
        .unwrap();

        // let center_coord = core::Point::new(
        // (frame_image.cols() - tip_size.width) / 2,
        // (frame_image.rows() - tip_size.height) / 2,
        // );
        let drawing_coord = if index == 0 {
            core::Point::new(text_coord.0, text_drawing_coord_y)
        } else {
            text_drawing_coord_y += tip_size.height + 10;
            core::Point::new(text_coord.0, text_drawing_coord_y)
        };

        let _ = imgproc::put_text(
            frame_image,
            temp_text,
            drawing_coord,
            imgproc::FONT_HERSHEY_DUPLEX,
            font_scale,
            core::Scalar::new(0f64, 255f64, 0f64, -1f64), // Border color (Blue, Green, Red, Alpha)
            font_thickness,
            imgproc::LINE_AA,
            false,
        )
        .unwrap();
    }
}

///
fn capture_from_web_cam() -> opencv::Result<()> {
    // Setup render window
    let window_name = "Web Cam Preview Window";
    let window_flags = highgui::WINDOW_AUTOSIZE
        | highgui::WINDOW_KEEPRATIO
        | highgui::WINDOW_OPENGL
        | highgui::WINDOW_NORMAL;
    highgui::named_window(window_name, window_flags).unwrap();

    // Create video capture (camera)
    #[cfg(not(feature = "opencv-32"))]
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

    // prelude::VideoCaptureTrait (open, open_file, is_opened, read, etc...)
    let is_cam_opened = videoio::VideoCapture::is_opened(&cam)?;
    if !is_cam_opened {
        panic!("Unable to open default web camera");
    }

    println!("Live camera is showing, press any key to close the app.");

    let mut grayscale_mode = false;

    loop {
        // Render every frame into preview window
        let mut video_frame = core::Mat::default()?;
        cam.read(&mut video_frame)?;
        if video_frame.size()?.width > 0 {
            draw_tips_on_frame(&mut video_frame);

            if grayscale_mode {
                let mut grayscale_frame = Mat::default()?;
                // Do a color conversion from `BRG(blue Red Gree 3 channels)` to `Grayscale`(1 channel)
                let _ = imgproc::cvt_color(
                    &video_frame,
                    &mut grayscale_frame,
                    imgproc::COLOR_BGR2GRAY,
                    0,
                )
                .unwrap();
                highgui::imshow(window_name, &grayscale_frame)?;
            } else {
                highgui::imshow(window_name, &video_frame)?;
            }
        }

        let key = highgui::wait_key(10)?;
        // if key > 0 {
        // println!("You pressed key: {}", &key);
        // }

        // Press `g` key to toggle `grayscale_mode`
        if key == 103 {
            grayscale_mode = !grayscale_mode;
            println!("Grayscale mode enabled: {}", grayscale_mode);
        }
        // Press any key to stop
        else if key > 0 && key != 255 {
            break;
        }
    }

    // Closes video file or capturing device.
    //
    // The method is automatically called by subsequent `VideoCapture::open` and by
    // `VideoCapture` destructor.
    //
    // The C function also deallocates memory and clears *capture pointer.
    cam.release()
}

///
fn main() {
    let close_capture_successfully = match capture_from_web_cam() {
        Ok(_) => true,
        Err(error) => {
            println!("Close video capture abnormally: {}", error);
            false
        }
    };

    let close_all_window_succesfully = match highgui::destroy_all_windows() {
        Ok(_) => true,
        Err(error) => {
            println!("Close all windows abnormally: {}", error);
            false
        }
    };

    if close_capture_successfully && close_all_window_succesfully {
        println!("Program exit normally:)");
    }
}
