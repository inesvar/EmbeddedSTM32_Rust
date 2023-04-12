use core::convert::AsMut;
use core::convert::AsRef;
use core::mem::transmute;
use core::ops::Div;
use core::ops::Index;
use core::ops::IndexMut;
use core::ops::Mul;
//use micromath::F32Ext;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const RED: Color = Color { r: 255, g: 0, b: 0 };
pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn base_color(color: u32) -> Color {
        let color = color%6;
        match color {
            0 => RED,
            1 => GREEN,
            2 => BLUE,
            3 => Color::new(255, 255, 0),
            4 => Color::new(0, 255, 255),
            5 => Color::new(255, 0, 255),
            _ => unreachable!(),
        }
    } 

    pub fn gamma_correct(&self) -> Self {
        Color::new(
            crate::gamma::gamma_correct(self.r),
            crate::gamma::gamma_correct(self.g),
            crate::gamma::gamma_correct(self.b),
        )
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color::new(mul(self.r, rhs), mul(self.g, rhs), mul(self.b, rhs))
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        Color::new(div(self.r, rhs), div(self.g, rhs), div(self.b, rhs))
    }
}

fn mul(a: u8, b: f32) -> u8 {
    let a2: f32 = a as f32;
    (a2 * b) as u8
}

fn div(a: u8, b: f32) -> u8 {
    let a2: f32 = a as f32;
    (a2 / b) as u8
}

#[repr(transparent)]
pub struct Image([Color; 64]);

impl Image {
    pub fn new_solid(color: Color) -> Self {
        Image([color; 64])
    }

    pub fn row(&self, row: usize) -> &[Color] {
        &self.0[row * 8..=row * 8 + 7]
    }

    pub fn gradient(color: Color) -> Self {
        let mut gradient: Image = Image::new_solid(color);
        for row in 0..8 {
            for col in 0..8 {
                gradient[(row, col)] = gradient[(row, col)] / (1 + row * row + col) as f32;
            }
        }
        gradient
    }

    pub fn gamma_correct(&self) -> Self {
        let mut corrected = Image::default();
        for row in 0..8 {
            for col in 0..8 {
                corrected[(row, col)] = self[(row, col)].gamma_correct();
            }
        }
        corrected
    }
}

impl Default for Image {
    fn default() -> Self {
        Image([Color::default(); 64])
    }
}

impl Index<(usize, usize)> for Image {
    type Output = Color;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0 * 8 + index.1]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0 * 8 + index.1]
    }
}

impl AsRef<[u8; 192]> for Image {
    fn as_ref(&self) -> &[u8; 192] {
        unsafe { transmute::<&Image, &[u8; 192]>(self) }
    }
}

impl AsMut<[u8; 192]> for Image {
    fn as_mut(&mut self) -> &mut [u8; 192] {
        unsafe { transmute::<&mut Image, &mut [u8; 192]>(self) }
    }
}