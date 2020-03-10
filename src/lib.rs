mod image;

pub use self::image::ClipImage;
use clip_sys::{
  clip_empty_format, clip_get_image, clip_get_paths, clip_get_text, clip_has, clip_image,
  clip_image_format, clip_path, clip_paths_format, clip_set_text, clip_text_format,
};
use failure::{err_msg, Error};
use std::path::PathBuf;

#[derive(Debug)]
pub enum ClipFormat {
  Empty,
  Text,
  Image,
  Paths,
}

pub struct Clip;

impl Clip {
  pub fn has_format(format: ClipFormat) -> bool {
    unsafe {
      let clip_format = match format {
        ClipFormat::Empty => clip_empty_format(),
        ClipFormat::Text => clip_text_format(),
        ClipFormat::Image => clip_image_format(),
        ClipFormat::Paths => clip_paths_format(),
      };

      clip_has(clip_format)
    }
  }

  pub fn get_format() -> Option<ClipFormat> {
    Some(if Clip::has_format(ClipFormat::Image) {
      ClipFormat::Image
    } else if Clip::has_format(ClipFormat::Paths) {
      ClipFormat::Paths
    } else if Clip::has_format(ClipFormat::Text) {
      ClipFormat::Text
    } else if Clip::has_format(ClipFormat::Empty) {
      ClipFormat::Empty
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

  pub fn get_paths() -> Result<Vec<PathBuf>, Error> {
    const PATHS_CAPACITY: usize = 100;

    let mut paths: [clip_path; PATHS_CAPACITY] = unsafe { std::mem::zeroed() };

    let paths_length = unsafe { clip_get_paths(paths.as_mut_ptr(), &(PATHS_CAPACITY as u64)) };
    assert!(paths_length > 0);

    println!("{}", paths_length);

    let paths: Vec<PathBuf> = paths[..paths_length as usize]
      .iter()
      .map(|path| {
        if path.wide {
          #[cfg(target_os = "windows")]
          {
            use std::os::windows::ffi::OsStringExt;

            let buf = unsafe { &path.buf.wide[..path.length as usize] };

            PathBuf::from(std::ffi::OsString::from_wide(buf))
          }

          #[cfg(not(target_os = "windows"))]
          {
            unimplemented!("path.wide");
          }
        } else {
          let buf = unsafe { &path.buf.ansi[..path.length as usize] };
          let buf = unsafe { &*(buf as *const [i8] as *const [u8]) };
          let buf = buf.to_vec();

          PathBuf::from(String::from_utf8(buf).unwrap())
        }
      })
      .collect();

    println!("{:#?}", paths);

    Ok(paths)
  }
}
