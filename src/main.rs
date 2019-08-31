extern crate num_complex;
extern crate image;

fn main() {
    let imgx = 3080;
    let imgy = 3080;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.1 * x as f32) as u8;
        let b = (0.2 * y as f32) as u8;
        let g = (0.4 * y as f32) as u8;
        // *pixel = image::Rgb([r, 0, b]);
    }

    let total = imgx * imgy;
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.54, 0.54);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 250 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }
            let ifl = i as f32 - (z.norm()).log2().log2();

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;

            let g =  ifl as u8;
            let r =  ifl as u8;
            let b =  z.norm() as u8;
            *pixel = image::Rgb([r,g,b]);
        }
        if x % 100 == 0 {
            println!("{}%", (x) as f32 / imgx as f32);
        }
    }
    imgbuf.save("fractal.ppm").unwrap();
    imgbuf.save("fractal.png").unwrap();
}
