use clip::*;

#[test]
fn test_text() {
  let s = "helloh".to_string();
  Clip::set_text(s.clone()).unwrap();

  assert_eq!(Clip::get_text().unwrap(), s);
}

#[test]
fn test_get_format() {
  println!("{:#?}", Clip::get_format().unwrap());
}

#[test]
fn test_get_image() {
  if !Clip::has_format(ClipFormat::Image) {
    eprintln!("skipping get_image test, no image in clipboard");
    return;
  }

  let clip_image = Clip::get_image().unwrap();
  println!("{:#?}", clip_image.get_spec());
}
