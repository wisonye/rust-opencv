use opencv::{core, highgui, imgproc, objdetect, prelude::*, types, videoio};
use std::{thread, time::Duration};

const TIPS: &'static str = "Press 'g' to toggle grayscale mode\nPress any key to exit";
const WINDOW_NAME: &'static str = "Web Cam Preview Window";

///
fn get_drawing_text_size<'a>(text: &'a str, font_scale: f64, font_thickness: i32) -> core::Size {
    let mut base_line = 0;
    imgproc::get_text_size(
        text,
        // FONT_HERSHEY_COMPLEX        - normal size serif font
        // FONT_HERSHEY_COMPLEX_SMALL  - smaller version of FONT_HERSHEY_COMPLEX
        // FONT_HERSHEY_DUPLEX         - normal size sans-serif font (more complex than FONT_HERSHEY_SIMPLEX)
        // FONT_HERSHEY_PLAIN          - small size sans-serif font
        // FONT_HERSHEY_SCRIPT_COMPLEX - more complex variant of FONT_HERSHEY_SCRIPT_SIMPLEX
        // FONT_HERSHEY_SCRIPT_SIMPLEX - hand-writing style font
        // FONT_HERSHEY_SIMPLEX        - normal size sans-serif font
        // FONT_HERSHEY_TRIPLEX        - normal size serif font (more complex than FONT_HERSHEY_COMPLEX)
        imgproc::FONT_HERSHEY_DUPLEX,
        font_scale,
        font_thickness,
        &mut base_line,
    )
    .unwrap()
}

///
fn draw_text_on_image<'a>(
    image: &mut Mat,
    text: &'a str,
    left_top_coord: core::Point,
    font_scale: f64,
    font_thickness: i32,
    text_color: core::Scalar,
) {
    let _ = imgproc::put_text(
        image,
        text,
        left_top_coord,
        imgproc::FONT_HERSHEY_DUPLEX,
        font_scale,
        text_color,
        font_thickness,
        imgproc::LINE_AA,
        false,
    )
    .unwrap();
}

///
fn draw_tips_on_frame(frame_image: &mut Mat) {
    let text_list = TIPS.split("\n");
    let text_coord = (5, 30); // X, Y
    let mut text_drawing_coord_y = text_coord.1;
    let font_scale = 0.8;
    let font_thickness = 1;

    for (index, temp_text) in text_list.enumerate() {
        let text_size = get_drawing_text_size(temp_text, font_scale, font_thickness);
        let drawing_coord = if index == 0 {
            core::Point::new(text_coord.0, text_drawing_coord_y)
        } else {
            text_drawing_coord_y += text_size.height + 10;
            core::Point::new(text_coord.0, text_drawing_coord_y)
        };

        draw_text_on_image(
            frame_image,
            temp_text,
            drawing_coord,
            font_scale,
            font_thickness,
            core::Scalar::new(0f64, 255f64, 0f64, -1f64), // Border color (Blue, Green, Red, Alpha)
        );
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
            4,                                            // Boarder thickness
            imgproc::LINE_AA,                             // Boarder line type
            0,
        )
        .unwrap();
    }

    // Render the frame after merging with drawing faces
    let _ = highgui::imshow(WINDOW_NAME, frame).unwrap();
}

///
fn draw_info_panel(
    frame: &mut Mat,
    frame_width: i32,
    frame_height: i32,
    fps: i32,
    detected_face_amount: u8,
) {
    let formatted_panel_info = format!(
        "Resolution: {} x {}\nFPS: {}\nDetected Faces: {}",
        frame_width, frame_height, fps, detected_face_amount
    );
    let text_list = formatted_panel_info.split("\n");
    let text_vertical_space = 10;
    let font_scale = 0.7;
    let font_thickness = 1;
    let font_color = core::Scalar::new(251., 235., 220., -1.); // (Blue, Green, Red, Alpha)
    let info_panel_background_color = core::Scalar::new(15., 6., 3., -1.); // (Blue, Green, Red, Alpha)
    let info_panel_width = 280;
    let info_panel_height = 88;
    let info_panel_margin = 2;

    // For getting the better performance, we create a `ROI`(Region Of Interest) from the origin
    // frame. This won't copy any data, as it's just a mut reference which will be affected if
    // we modify it!!!
    let roi = core::Rect {
        x: frame_width - info_panel_width - info_panel_margin,
        y: info_panel_margin,
        width: info_panel_width,
        height: info_panel_height,
    };
    let mut panel_roi_ref = core::Mat::roi(frame, roi).unwrap();

    // Create a temp draw area with the same size of `roi`
    let mut panel_background = core::Mat::new_size_with_default(
        core::Size {
            width: panel_roi_ref.cols(),
            height: panel_roi_ref.rows(),
        },
        panel_roi_ref.typ().unwrap(),
        core::Scalar::new(0., 0., 0., -1.),
    )
    .unwrap();
    let panel_background_area = core::Rect {
        x: 0,
        y: 0,
        width: roi.width,
        height: roi.height,
    };
    // println!("roi: {:#?}", roi);
    // println!("panel_roi_ref: {:#?}", panel_roi_ref);
    // println!("panel_background_area: {:#?}", panel_background_area);
    // println!("panel_background: {:#?}", panel_background);

    // Fill the color
    let _ = imgproc::rectangle(
        &mut panel_background,       // Dest image
        panel_background_area,       // Rectangle to draw
        info_panel_background_color, // Boarder color
        imgproc::FILLED,             // Boarder thickness: Fill the entire area
        imgproc::LINE_AA,            // Boarder line type
        0,
    );

    // Merge `panel_roi_ref` and `panel_background` together with the particular alpha(transparent)
    // settings. So, we finished drawing a transparent background on top of the original frame:)
    //
    // `src image alpha` + `copy image alpha` should equal `1.0`. Just like a transparent percentage.
    let _ = core::add_weighted(
        &panel_roi_ref.clone(), // Src image
        0.3,                    // Src image alpha
        &panel_background,      // Copy image
        0.7,                    // Copy image alpha
        0.,                     // Gamma
        &mut panel_roi_ref,     // The merge dest image
        -1,
    );

    // Draw all split text
    let text_coord = (roi.x + 6, roi.y + 25); // X, Y
    let mut text_drawing_coord_y = text_coord.1;
    for (index, temp_text) in text_list.enumerate() {
        let text_size = get_drawing_text_size(temp_text, font_scale, font_thickness);

        let drawing_coord = if index == 0 {
            core::Point::new(text_coord.0, text_drawing_coord_y)
        } else {
            text_drawing_coord_y += text_size.height + text_vertical_space;
            core::Point::new(text_coord.0, text_drawing_coord_y)
        };

        draw_text_on_image(
            frame,
            temp_text,
            drawing_coord,
            font_scale,
            font_thickness,
            font_color,
        );
    }
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

    // Create object detection classifier
    let mut face = objdetect::CascadeClassifier::new(&xml)?;

    let mut grayscale_mode = false;

    let cam_width = cam.get(videoio::CAP_PROP_FRAME_WIDTH).unwrap();
    let cam_height = cam.get(videoio::CAP_PROP_FRAME_HEIGHT).unwrap();
    let cam_fps = cam.get(videoio::CAP_PROP_FPS).unwrap();

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

        // Draw info panel
        draw_info_panel(
            &mut video_frame,
            cam_width as i32,
            cam_height as i32,
            cam_fps as i32,
            detected_faces.len() as u8,
        );

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
