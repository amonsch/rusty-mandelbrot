extern crate num;

use num::complex::Complex;

fn main() {
    let width_px = 1200.0;
    let height_px = width_px / 3.0 * 2.0;

    println! {"P3"};
    println! {"{} {}", width_px, height_px};
    println! {"255"}

    let step_x: f32 = 3.0 / width_px;
    let step_y: f32 = 2.0 / height_px;

    let mut zn: f32;
    for y in (0..height_px as i32).rev() {
        'inner: for x in 0..width_px as i32 {
            let c = Complex {
                re: x as f32 * step_x - 2.0,
                im: y as f32 * step_y - 1.0,
            };
            let mut z = Complex { re: 0.0, im: 0.0 };
            for n in 0..255 {
                z = z * z + c;
                zn = z.norm_sqr();
                if zn > 4.0 {
                    println!("{} {} {}", zn as u8, (n as f32 - zn).abs() as u8, zn as u8);
                    continue 'inner;
                }
            }
            println!("0 0 0");
        }
    }
}
