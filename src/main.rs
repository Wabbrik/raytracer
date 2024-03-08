use std::{
    fmt::{Debug, Display},
    fs::File,
    io::{BufWriter, Write},
};

mod vec3;

fn main() -> std::io::Result<()> {
    let mut buffer = BufWriter::new(File::create("out.ppm")?);
    let img = to_draw(256, 256);

    write!(buffer, "{}", Ppm(&img))?;
    buffer.flush()?;
    Ok(())
}

fn to_draw(width: usize, height: usize) -> Image {
    let factor: f32 = 259.999;

    Image::new_assign(width, height, |i, j| {
        let r = (i as f32) / ((width - 1) as f32);
        let g = (j as f32) / ((height - 1) as f32);

        Pixel {
            r: (factor * r) as u8,
            g: (factor * g) as u8,
            b: 0,
        }
    })
}

#[derive(Default, Debug)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

struct Image {
    pixels: Vec<Vec<Pixel>>,
    height: usize,
    width: usize,
}

impl Image {
    pub fn new(height: usize, width: usize) -> Self {
        let pixels: Vec<Vec<Pixel>> = (0..height)
            .map(|_| (0..width).map(|_| Pixel::default()).collect())
            .collect();

        Self {
            pixels,
            height,
            width,
        }
    }

    pub fn new_assign(
        height: usize,
        width: usize,
        pix_init: impl Fn(usize, usize) -> Pixel,
    ) -> Self {
        let pixels = (0..height)
            .map(|row| (0..width).map(|col| pix_init(row, col)).collect())
            .collect();

        Self {
            pixels,
            height,
            width,
        }
    }
}

struct Ppm<'a, T>(&'a T);

impl Display for Ppm<'_, Pixel> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:>3} {:>3} {:>3}", self.0.r, self.0.g, self.0.b)
    }
}

impl Debug for Ppm<'_, Pixel> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self)
    }
}

impl Display for Ppm<'_, Image> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(formatter, "P3")?; // magic number
        writeln!(formatter, "{} {}", self.0.width, self.0.height)?;
        writeln!(formatter, "255")?; // maximum color value

        for row in &self.0.pixels {
            for pixel in row {
                writeln!(formatter, "{}", Ppm(pixel))?;
            }
        }
        Ok(())
    }
}

impl Debug for Ppm<'_, Image> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self)
    }
}

#[cfg(test)]
mod test_ppm {
    use super::*;

    #[test]
    fn test_pixel_print() {
        let pixel = Pixel { r: 42, g: 0, b: 0 };
        k9::snapshot!(Ppm(&pixel), " 42   0   0");
    }

    #[test]
    fn test_image_init() {
        let img = Image::new_assign(2, 2, |i, j| Pixel {
            r: i as u8,
            g: j as u8,
            b: i as u8,
        });
        k9::snapshot!(
            Ppm(&img),
            "
P3
2 2
255
  0   0   0
  0   1   0
  1   0   1
  1   1   1

"
        );
    }
}
