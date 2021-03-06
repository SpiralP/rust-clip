use clip_sys::{clip_image, clip_image_spec};
use failure::{err_msg, Error};
use image::{bmp::BMPEncoder, jpeg::JPEGEncoder, png::PNGEncoder, ColorType, ImageError};
use std::{io::Write, slice};

pub struct ClipImage {
  clip_image: clip_image,
}

pub trait Encoder {
  fn encode(
    self,
    data: &[u8],
    width: u32,
    height: u32,
    color_type: ColorType,
  ) -> Result<(), ImageError>;
}

impl<W: Write> Encoder for PNGEncoder<W> {
  fn encode(
    self,
    data: &[u8],
    width: u32,
    height: u32,
    color_type: ColorType,
  ) -> Result<(), ImageError> {
    PNGEncoder::encode(self, data, width, height, color_type)
  }
}

impl<'a, W: 'a> Encoder for JPEGEncoder<'a, W>
where
  W: Write,
{
  fn encode(
    mut self,
    data: &[u8],
    width: u32,
    height: u32,
    color_type: ColorType,
  ) -> Result<(), ImageError> {
    JPEGEncoder::encode(&mut self, data, width, height, color_type)
  }
}

impl<'a, W: 'a> Encoder for BMPEncoder<'a, W>
where
  W: Write,
{
  fn encode(
    mut self,
    data: &[u8],
    width: u32,
    height: u32,
    color_type: ColorType,
  ) -> Result<(), ImageError> {
    BMPEncoder::encode(&mut self, data, width, height, color_type)
  }
}

impl ClipImage {
  pub(crate) fn from_clip_image(clip_image: clip_image) -> Self {
    Self { clip_image }
  }

  pub fn get_spec(&self) -> &clip_image_spec {
    self.clip_image.spec()
  }

  pub fn get_data(&self) -> &[u8] {
    let spec = self.get_spec();
    let len: usize = spec.bytes_per_row as usize * spec.height as usize;
    unsafe { slice::from_raw_parts(self.clip_image.data(), len) }
  }

  pub fn write_png<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
    self.write_from_encoder(PNGEncoder::new(writer))
  }

  pub fn write_jpeg<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
    self.write_from_encoder(JPEGEncoder::new(writer))
  }

  pub fn write_bmp<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
    self.write_from_encoder(BMPEncoder::new(writer))
  }

  pub fn write_from_encoder<E: Encoder>(&self, encoder: E) -> Result<(), Error> {
    let clip_spec = self.get_spec();
    let clip_data = self.get_data();

    let width = clip_spec.width as u32;
    let height = clip_spec.height as u32;

    let color_type = color_type_from_spec(&clip_spec)
      .ok_or_else(|| err_msg("color_type_from_spec unhandled spec type"))?;

    let bits_per_pixel = clip_spec.bits_per_pixel;
    let bytes_in_pixel = (bits_per_pixel / 8) as usize;

    // our png/jpeg encoders only support RGB, so we convert BGR
    match color_type {
      ColorType::Bgra8 => {
        let mut clip_data = clip_data.to_vec();

        for i in (0..clip_data.len()).step_by(bytes_in_pixel) {
          clip_data.swap(i, i + 2); // swap blue and red
        }

        encoder.encode(&clip_data, width, height, ColorType::Rgba8)?;
      }

      ColorType::Bgr8 => {
        let mut clip_data = clip_data.to_vec();

        for i in (0..clip_data.len()).step_by(bytes_in_pixel) {
          clip_data.swap(i, i + 2); // swap blue and red
        }

        encoder.encode(&clip_data, width, height, ColorType::Rgb8)?;
      }

      _ => {
        encoder.encode(&clip_data, width, height, color_type)?;
      }
    }

    Ok(())
  }
}

fn color_type_from_spec(spec: &clip_image_spec) -> Option<ColorType> {
  Some(
    match (
      spec.red_shift,
      spec.green_shift,
      spec.blue_shift,
      spec.alpha_shift,
    ) {
      // R G B A
      (0, 8, 16, 24) => match spec.bits_per_pixel / 4 {
        8 => ColorType::Rgba8,
        16 => ColorType::Rgba16,
        _ => unreachable!(),
      },
      // R G B
      (0, 8, 16, 0) => match spec.bits_per_pixel / 3 {
        8 => ColorType::Rgb8,
        16 => ColorType::Rgb16,
        _ => unreachable!(),
      },

      // B G R A
      (16, 8, 0, 24) => match spec.bits_per_pixel / 4 {
        8 => ColorType::Bgra8,
        _ => unreachable!(),
      },

      // B G R
      (16, 8, 0, 0) => match spec.bits_per_pixel / 3 {
        8 => ColorType::Bgr8,
        _ => unreachable!(),
      },

      _ => return None,
    },
  )
}
