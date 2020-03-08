mod image;

pub use self::image::ClipImage;
use clip_sys::{
  clip_empty_format, clip_get_image, clip_get_text, clip_has, clip_image, clip_image_format,
  clip_set_text, clip_text_format,
};
use failure::{err_msg, Error};

#[derive(Debug)]
pub enum ClipFormat {
  Empty,
  Text,
  Image,
}

pub struct Clip;

impl Clip {
  pub fn has_format(format: ClipFormat) -> bool {
    unsafe {
      let clip_format = match format {
        ClipFormat::Empty => clip_empty_format(),
        ClipFormat::Text => clip_text_format(),
        ClipFormat::Image => clip_image_format(),
      };

      clip_has(clip_format)
    }
  }

  pub fn get_format() -> Option<ClipFormat> {
    Some(if Clip::has_format(ClipFormat::Empty) {
      ClipFormat::Empty
    } else if Clip::has_format(ClipFormat::Text) {
      ClipFormat::Text
    } else if Clip::has_format(ClipFormat::Image) {
      ClipFormat::Image
    } else {
      return None;
    })
  }

  pub fn set_text(text: String) -> Result<(), Error> {
    use std::ffi::CString;

    let c_string = CString::new(text)?;

    let did_set = unsafe { clip_set_text(c_string.as_ptr()) };

    if !did_set {
      Err(err_msg("couldn't set clipboard text"))
    } else {
      Ok(())
    }
  }

  pub fn get_text() -> Result<String, Error> {
    Ok(unsafe { clip_get_text() }.to_string()?)
  }

  pub fn get_image() -> Result<ClipImage, Error> {
    let mut img = unsafe { clip_image::new() };

    let ok = unsafe { clip_get_image(&mut img) };
    if !ok {
      return Err(err_msg("couldn't get image"));
    }

    Ok(ClipImage::from_clip_image(img))
  }
}
