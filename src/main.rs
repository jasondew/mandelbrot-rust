extern crate image;
extern crate num_complex;

type Complex = num_complex::Complex32;

fn escape_time(c: &Complex) -> Option<u8> {
    let z: &mut Complex = &mut Complex::new(0.0, 0.0);
    let i: &mut u8 = &mut 1;

    loop {
        if *i >= std::u8::MAX {
            break None;
        }

        *z = *z * *z + *c;
        if z.l1_norm() > 2.0 {
            break Some(*i);
        }
        *i += 1;
    }
}

fn main() -> std::io::Result<()> {
    let from_x: f32 = -1.75;
    let to_x: f32 = 0.5;
    let from_y: f32 = -1.0;
    let to_y: f32 = 1.0;

    let scale: f32 = 0.0001;

    let width: u32 = ((to_x - from_x) / scale) as u32;
    let height: u32 = ((to_y - from_y) / scale) as u32;

    image::ImageBuffer::from_fn(width, height, |x, y| {
        match escape_time(&Complex::new(
            (x as f32) * scale + from_x,
            (y as f32) * scale + from_y,
        )) {
            Some(i) => image::Luma([i]),
            None => image::Luma([0u8]),
        }
    })
    .save("mandelbrot.png")
    .expect("Unable to write data to file");

    Ok(())
}
