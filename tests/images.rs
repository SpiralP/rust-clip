use clip::*;
use std::fs::File;

#[test]
fn test_write_image_as_png() {
  if !Clip::has_format(ClipFormat::Image) {
    eprintln!("skipping png test, no image in clipboard");
    return;
  }

  let image = Clip::get_image().unwrap();
  image
    .write_png(&mut File::create("test.png").unwrap())
    .unwrap();
}

#[test]
fn test_write_image_as_jpeg() {
  if !Clip::has_format(ClipFormat::Image) {
    eprintln!("skipping jpeg test, no image in clipboard");
    return;
  }

  let image = Clip::get_image().unwrap();
  image
    .write_jpeg(&mut File::create("test.jpeg").unwrap())
    .unwrap();
}
