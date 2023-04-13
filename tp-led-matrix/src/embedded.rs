use embedded_graphics::{Pixel, Drawable};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{DrawTarget, OriginDimensions, RgbColor, Size, Point};
use embedded_graphics::primitives::{Rectangle, StyledDrawable, PrimitiveStyle};
use embedded_graphics::text::Text;
use ibm437::IBM437_8X8_REGULAR;
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
    pub fn color_cycle(color: u32) -> Color {
        match color {
        // hues precised
        // chroma : 150
        // lightness : 80
        // using https://jackw01.github.io/HCLPicker/
        // and then rgb divided by 5
            0 => Color::new(51, 0, 51), //340
            1 => Color::new(51, 0, 23), //20
            2 => Color::new(51, 0, 8), //35
            3 => Color::new(51, 13, 0), //47
            4 => Color::new(51, 25, 0), //60
            5 => Color::new(51, 36, 0), //82
            6 => Color::new(40, 42, 0), //100
            7 => Color::new(24, 45, 0), //115
            8 => Color::new(2, 46, 0),
            9 => Color::new(0, 49, 12), //155
            10 => Color::new(0, 50, 35),
            11 => Color::new(0, 35, 39), 
            12 => Color::new(0, 15, 51), 
            13 => Color::new(17, 5, 45), 
            14 => Color::new(35, 10, 51),
            //unused in the cycle
            15 => Color::new(51, 51, 51), 
            _ => unreachable!(),
        }
    } 
}

impl Image {
    pub fn draw_shape(shape: u32, color1: u32, color2: u32, color3: u32, color4: u32) -> Self {
        let color1 = Color::color_cycle(color1).into();
        let color2 = Color::color_cycle(color2).into();
        let color3 = Color::color_cycle(color3).into();
        let color4 = Color::color_cycle(color4).into();
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

    pub fn show_text(text: &str, index: u32, color: u32) -> Self {
        let mut image = Image::default();
        let width: u32 = text.chars().count() as u32 * 8 + 8;
        let index: i32 = (index%width).try_into().unwrap();
        let point = Point::new(8-index, 6); 
        let color: Rgb888 = Color::color_cycle(color).into();
        let text = Text::new(text, point, 
                    MonoTextStyle::new(&IBM437_8X8_REGULAR, color));
        text.draw(&mut image).unwrap();
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
                    self[(coord.y as usize, coord.x as usize)] = color.into();
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


