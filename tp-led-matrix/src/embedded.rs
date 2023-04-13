use embedded_graphics::Pixel;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{DrawTarget, OriginDimensions, RgbColor, Size, Point};
use embedded_graphics::primitives::{Rectangle, StyledDrawable, PrimitiveStyle};
use core::convert::Infallible;
use crate::image::*;

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

impl Color {
    pub fn purple_to_green_shades(color: u32) -> Color {
        match color {
        // hues : from 0 to 160 and back
        // chroma : 150
        // lightness : 80
        // and then rgb divided by 5
            0 => Color::new(51, 0, 41), //0
            1 => Color::new(51, 0, 8), //35
            2 => Color::new(51, 13, 0), //47
            3 => Color::new(51, 25, 0), //60
            4 => Color::new(51, 35, 0), //80
            5 => Color::new(35, 43, 0), //105
            6 => Color::new(16, 26, 0), //120
            7 => Color::new(0, 48, 0), //140
            8 => Color::new(0, 49, 18), //160
            9 => Color::new(0, 48, 0), //140
            10 => Color::new(16, 26, 0), //120
            11 => Color::new(35, 43, 0), //105
            12 => Color::new(51, 35, 0), //80
            13 => Color::new(51, 25, 0), //60
            14 => Color::new(51, 13, 0), //47
            15 => Color::new(51, 0, 8), //35
            _ => unreachable!(),
        }
    } 
}

impl Image {
    pub fn draw_shape(shape: u32, color1: u32, color2: u32, color3: u32, color4: u32) -> Self {
        let color1 = Color::purple_to_green_shades(color1).into();
        let color2 = Color::purple_to_green_shades(color2).into();
        let color3 = Color::purple_to_green_shades(color3).into();
        let color4 = Color::purple_to_green_shades(color4).into();
        let mut image = Image::default();
        match shape {
            0 => {let rectangle= Rectangle::new(Point::new(5, 5), Size::new(3,3));
                rectangle.draw_styled(&PrimitiveStyle::with_stroke(color1, 1), &mut image).unwrap();
                let rectangle= Rectangle::new(Point::new(3, 3), Size::new(3, 3));
                rectangle.draw_styled(&PrimitiveStyle::with_stroke(color2, 1), &mut image).unwrap();
                let rectangle= Rectangle::new(Point::new(1, 1), Size::new(3, 3));
                rectangle.draw_styled(&PrimitiveStyle::with_stroke(color3, 1), &mut image).unwrap();
                let rectangle= Rectangle::new(Point::new(-1, -1), Size::new(3, 3));
                rectangle.draw_styled(&PrimitiveStyle::with_stroke(color4, 1), &mut image).unwrap();},
            1 => {let rectangle= Rectangle::new(Point::new(6, 6), Size::new(3, 3));
                rectangle.draw_styled(&PrimitiveStyle::with_stroke(color1, 1), &mut image).unwrap();
                let rectangle= Rectangle::new(Point::new(4, 4), Size::new(3, 3));
                rectangle.draw_styled(&PrimitiveStyle::with_stroke(color2, 1), &mut image).unwrap();
                let rectangle= Rectangle::new(Point::new(2, 2), Size::new(3, 3));
                rectangle.draw_styled(&PrimitiveStyle::with_stroke(color3, 1), &mut image).unwrap();
                let rectangle= Rectangle::new(Point::new(0, 0), Size::new(3, 3));
                rectangle.draw_styled(&PrimitiveStyle::with_stroke(color4, 1), &mut image).unwrap();},
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


