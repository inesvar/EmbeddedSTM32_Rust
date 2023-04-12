use embedded_graphics::Pixel;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{DrawTarget, OriginDimensions, RgbColor, Size, Point};
use embedded_graphics::primitives::{Rectangle, Circle, Line, StyledDrawable, PrimitiveStyle};
use core::convert::Infallible;
use crate::image::{Color, Image};

impl From<Rgb888> for Color {
    fn from(rgb888: Rgb888) -> Self { 
        Color::new(rgb888.r(), rgb888.g(), rgb888.b())
     }
}

impl From<Color> for Rgb888 {
    fn from(color: Color) -> Self { 
        Rgb888::new(color.r, color.g, color.b)
     }
}

impl Image {
    pub fn draw_shape(shape: u32, color: Rgb888) -> Self {
        let mut image = Image::default();
        let shape = (shape / 6)%3;
        match shape {
            0 => {let line = Line::new(Point::new(0, 0), Point::new(7, 7));
                line.draw_styled(&PrimitiveStyle::with_stroke(color, 1), &mut image).unwrap();},
            1 => {let rectangle= Rectangle::new(Point::new(2, 1), Size::new(4,6));
                    rectangle.draw_styled(&PrimitiveStyle::with_stroke(color, 1), &mut image).unwrap();},
            2 => {let circle = Circle::new(Point::new(1, 1), 6);
                        circle.draw_styled(&PrimitiveStyle::with_stroke(color, 1), &mut image).unwrap();},
            _ => unreachable!(),
        }
        image
    }
}

impl DrawTarget for Image {
    type Color = Rgb888;
    type Error = Infallible;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where 
            I: IntoIterator<Item = Pixel<Self::Color>> {
                
        for Pixel(coord, color) in pixels.into_iter() {
            if 0 <= coord.x && coord.x < 8 {
                if 0 <= coord.y && coord.y < 8 {
                    self[(coord.x as usize, coord.y as usize)] = color.into();
                }
            }
        }
        Ok(())
    }
}

impl OriginDimensions for Image {
    fn size(&self) -> Size {
        Size::new(8, 8)
    }
}


