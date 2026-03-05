/*
Author: Rajesh Pachaikani
Contact: rajesh@makeitnow.in
*/

use opencv::{highgui, prelude::*, videoio, Result};

fn main() -> Result<()> {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
    let mut frame = Mat::default();
    loop{
        cam.read(&mut frame)?;
        highgui::imshow("window", &frame)?;
        let key = highgui::wait_key(1)?;
        if key == 113{
            break;
        }
    }

    Ok(())
}
