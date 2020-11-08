use opencv::{core, highgui, imgproc, objdetect, prelude::*, types, videoio};
use std::{thread, time::Duration};

const TIPS: &'static str = "Press 'g' to toggle grayscale mode\nPress any key to exit";
const WINDOW_NAME: &'static str = "Web Cam Preview Window";

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
fn face_detection_on_frame(
    face: &mut objdetect::CascadeClassifier,
    frame: &Mat,
) -> opencv::Result<core::Vector<core::Rect>> {
    // Convert every frame into gray color
    let mut gray = Mat::default()?;
    imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    // Reduce the image size for fast face detection
    let mut reduced = Mat::default()?;
    imgproc::resize(
        &gray,
        &mut reduced,
        core::Size {
            width: 0,
            height: 0,
        },
        0.25f64,
        0.25f64,
        imgproc::INTER_LINEAR,
    )?;

    // Run face detection
    let mut detected_faces = types::VectorOfRect::new();
    face.detect_multi_scale(
        &reduced,
        &mut detected_faces,
        1.1,
        2,
        objdetect::CASCADE_SCALE_IMAGE,
        core::Size {
            width: 30,
            height: 30,
        },
        core::Size {
            width: 0,
            height: 0,
        },
    )?;

    Ok(detected_faces)
}

///
fn draw_detected_faces_on_frame(frame: &mut Mat, faces: core::Vector<core::Rect>) {
    for temp_face in faces {
        // println!("temp_face: {:#?}", temp_face);

        let scaled_face = core::Rect {
            x: temp_face.x * 4,
            y: temp_face.y * 4,
            width: temp_face.width * 4,
            height: temp_face.height * 4,
        };

        let _ = imgproc::rectangle(
            frame,                                        // Dest image
            scaled_face,                                  // Rectangle to draw
            core::Scalar::new(0f64, 0f64, 255f64, -1f64), // Border color (Blue, Green, Red, Alpha)
            5,                                            // Boarder thickness
            8,                                            // Boarder line type
            0,
        )
        .unwrap();
    }

    // Render the frame after merging with drawing faces
    let _ = highgui::imshow(WINDOW_NAME, frame).unwrap();
}

///
fn capture_from_web_cam_with_face_detection() -> opencv::Result<()> {
    let window_flags = highgui::WINDOW_AUTOSIZE
        | highgui::WINDOW_KEEPRATIO
        | highgui::WINDOW_OPENGL
        | highgui::WINDOW_NORMAL;
    highgui::named_window(WINDOW_NAME, window_flags).unwrap();

    // Create video capture (camera), `0` means default webcam.
    // You can pass `1` for the second camera, `2` for the third camera.
    // Load face detection settings
    #[cfg(not(feature = "opencv-32"))]
    let (xml, mut cam) = {
        (
            core::find_file("haarcascades/haarcascade_frontalface_alt.xml", true, false)?,
            videoio::VideoCapture::new(0, videoio::CAP_ANY)?,
        )
    };

    let is_cam_opened = videoio::VideoCapture::is_opened(&cam)?;
    if !is_cam_opened {
        panic!("Unable to open default web camera");
    }

    println!("Live camera is showing, press any key to close the app.");

    // Create object detection classifier
    let mut face = objdetect::CascadeClassifier::new(&xml)?;

    let mut grayscale_mode = false;

    loop {
        // Read every frame
        let mut video_frame = core::Mat::default()?;
        cam.read(&mut video_frame)?;
        if video_frame.size()?.width == 0 {
            thread::sleep(Duration::from_secs(5));
            continue;
        }

        // Draw tips
        draw_tips_on_frame(&mut video_frame);

        // Do face detection
        let detected_faces = face_detection_on_frame(&mut face, &mut video_frame).unwrap();

        // println!("Detected face amount: {}", faces.len());

        // Draw a rectangle for each face result on top of the particular (frame) image
        if grayscale_mode {
            let mut grayscale_frame = Mat::default()?;
            // Do a color conversion from `BRG(blue Red Green 3 channels)` to `Grayscale`(1 channel)
            let _ = imgproc::cvt_color(
                &video_frame,
                &mut grayscale_frame,
                imgproc::COLOR_BGR2GRAY,
                0,
            )
            .unwrap();
            draw_detected_faces_on_frame(&mut grayscale_frame, detected_faces);
        } else {
            draw_detected_faces_on_frame(&mut video_frame, detected_faces);
        }

        let key = highgui::wait_key(10)?;

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
    let close_capture_successfully = match capture_from_web_cam_with_face_detection() {
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
