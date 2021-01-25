extern crate opencv;

use opencv::{
    prelude::*,
    imgproc,
    highgui
};


pub fn display_img(image: &Mat, title: Option<&str>) -> opencv::Result<()> {
    let t = title.unwrap_or ("window");
    highgui::named_window(t, 0)?;
    highgui::imshow(t, image)?;
    highgui::wait_key(10000)?;

    Ok(())
}

pub fn empty(mat: &Mat) -> opencv::Result<bool> {
    let size = mat.size()?;
    Ok(size.width == 0 && size.height == 0)
}

pub fn enhance_frame (frame : &Mat)
                  -> opencv::Result<Mat>{
    let mut gray = Mat::default()?;
    let mut equalized = Mat::default()?;

    imgproc::cvt_color(
        &frame,
        &mut gray,
        imgproc::COLOR_BGR2GRAY,
        0
    )?;

    imgproc::equalize_hist (&gray,
                            &mut equalized)?;

    Ok(equalized)
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

