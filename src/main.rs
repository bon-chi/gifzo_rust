extern crate image;
extern crate screenshot;
use screenshot::get_screenshot;
use std::path::Path;

fn main() {
    let s = get_screenshot(0).unwrap();

    println!("{} x {}", s.width(), s.height());

    image::save_buffer(Path::new("test.png"),
                       s.as_ref(),
                       s.width() as u32,
                       s.height() as u32,
                       image::RGBA(8))
        .unwrap();
}
