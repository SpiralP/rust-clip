mod clip_image;

pub use self::clip_image::ClipImage;
use clip_sys::{
  clip_delete_text, clip_empty_format, clip_get_image, clip_get_text, clip_has, clip_image_format,
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

    let did_set = unsafe {
      let raw = c_string.into_raw();

      let did_set = clip_set_text(raw);

      CString::from_raw(raw);

      did_set
    };

    if !did_set {
      Err(err_msg("couldn't set clipboard text"))
    } else {
      Ok(())
    }
  }

  pub fn get_text() -> Result<String, Error> {
    use std::ffi::CStr;

    unsafe {
      let c_str = clip_get_text();
      if c_str.is_null() {
        Err(err_msg("couldn't get clipboard text"))
      } else {
        let string = CStr::from_ptr(c_str)
          .to_str()
          .map(std::string::ToString::to_string)
          .map_err(std::convert::Into::into);

        clip_delete_text(c_str);

        string
      }
    }
  }

  pub fn get_image() -> Result<ClipImage, Error> {
    unsafe {
      let ptr = clip_get_image();
      if ptr.is_null() {
        Err(err_msg("couldn't get clipboard image"))
      } else {
        Ok(ClipImage::from_ptr(ptr))
      }
    }
  }
}
