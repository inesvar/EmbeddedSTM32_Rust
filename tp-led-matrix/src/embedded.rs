use embedded_graphics::Pixel;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{DrawTarget, OriginDimensions, RgbColor, Size};
use core::convert::Infallible;
use crate::image::{Color, Image};

impl From<Rgb888> for Color {
    fn from(rgb888: Rgb888) -> Self { 
        Color::new(rgb888.r(), rgb888.g(), rgb888.b())
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


