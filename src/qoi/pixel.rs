use super::operator::Operator;

pub trait Pixel
where
    Self: Sized + Copy,
{
    fn initial_array() -> [Self; 64];
    fn initial_pixel() -> Self;

    fn apply_diff(&mut self, dr: u8, dg: u8, db: u8);
    fn set_rgb(&mut self, r: u8, g: u8, b: u8);
    fn set_rgba(&mut self, r: u8, g: u8, b: u8, a: u8);
    fn index(&self) -> usize;

    fn write_to_vec(&self, v: &mut Vec<u8>);
}

fn calculate_index(r: u8, g: u8, b: u8, a: u8) -> usize {
    (r as usize * 3 + g as usize * 5 + b as usize * 7 + a as usize * 11) % 64usize
}

#[derive(Clone, Copy)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel for Rgb {
    fn initial_array() -> [Self; 64] {
        [Self { r: 0, g: 0, b: 0 }; 64]
    }

    fn initial_pixel() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    fn apply_diff(&mut self, dr: u8, dg: u8, db: u8) {
        self.r = self.r.wrapping_add(dr);
        self.g = self.g.wrapping_add(dg);
        self.b = self.b.wrapping_add(db);
    }

    fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        *self = Self { r, g, b };
    }

    fn set_rgba(&mut self, r: u8, g: u8, b: u8, _a: u8) {
        *self = Self { r, g, b };
    }

    fn index(&self) -> usize {
        calculate_index(self.r, self.g, self.b, 255)
    }

    fn write_to_vec(&self, v: &mut Vec<u8>) {
        v.extend_from_slice(&[self.r, self.g, self.b]);
    }
}

#[derive(Clone, Copy)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel for Rgba {
    fn initial_array() -> [Self; 64] {
        [Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }; 64]
    }

    fn initial_pixel() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    fn apply_diff(&mut self, dr: u8, dg: u8, db: u8) {
        self.r = self.r.wrapping_add(dr);
        self.g = self.g.wrapping_add(dg);
        self.b = self.b.wrapping_add(db);
    }

    fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        *self = Self { r, g, b, a: self.a };
    }

    fn set_rgba(&mut self, r: u8, g: u8, b: u8, a: u8) {
        *self = Self { r, g, b, a };
    }

    fn index(&self) -> usize {
        calculate_index(self.r, self.g, self.b, self.a)
    }

    fn write_to_vec(&self, v: &mut Vec<u8>) {
        v.extend_from_slice(&[self.r, self.g, self.b, self.a]);
    }
}

pub fn process_operators<I: Iterator<Item = Operator>, P: Pixel>(input: I) -> Vec<u8> {
    let mut output = Vec::new();

    let mut previous_array = P::initial_array();
    let mut pixel = P::initial_pixel();

    for op in input {
        let n = match op {
            Operator::Index(idx) => {
                pixel = previous_array[idx as usize];
                1
            }
            Operator::Diff {
                diff_red,
                diff_green,
                diff_blue,
            } => {
                pixel.apply_diff(diff_red, diff_green, diff_blue);
                1
            }
            Operator::Run(length) => length,
            Operator::Rgb(r, g, b) => {
                pixel.set_rgb(r, g, b);
                1
            }
            Operator::Rgba(r, g, b, a) => {
                pixel.set_rgba(r, g, b, a);
                1
            }
        };

        previous_array[pixel.index()] = pixel;
        for _ in 0..n {
            pixel.write_to_vec(&mut output);
        }
    }

    output
}
