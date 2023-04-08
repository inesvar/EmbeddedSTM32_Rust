use stm32l4xx_hal::{gpio::*, rcc::Clocks};
use stm32l4xx_hal::delay::DelayCM as Delay;
use stm32l4xx_hal::prelude::_embedded_hal_blocking_delay_DelayMs as DelayMs;
use crate::{Color, Image};

pub struct Matrix {
    sb: PC5<Output<PushPull>>,
    lat: PC4<Output<PushPull>>,
    rst: PC3<Output<PushPull>>,
    sck: PB1<Output<PushPull>>,
    sda: PA4<Output<PushPull>>,
    c0: PB2<Output<PushPull>>,
    c1: PA15<Output<PushPull>>,
    c2: PA2<Output<PushPull>>,
    c3: PA7<Output<PushPull>>,
    c4: PA6<Output<PushPull>>,
    c5: PA5<Output<PushPull>>,
    c6: PB0<Output<PushPull>>,
    c7: PA3<Output<PushPull>>,
}

impl Matrix {
    /// Create a new matrix from the control registers and the individual
    /// unconfigured pins. SB and LAT will be set high by default, while
    /// other pins will be set low. After 100ms, RST will be set high, and
    /// the bank 0 will be initialized by calling `init_bank0()` on the
    /// newly constructed structure.
    /// The pins will be set to very high speed mode.
    #[allow(clippy::too_many_arguments)]   // Necessary to avoid a clippy warning
    pub fn new(
        pa2: PA2<Analog>,
        pa3: PA3<Analog>,
        pa4: PA4<Analog>,
        pa5: PA5<Analog>,
        pa6: PA6<Analog>,
        pa7: PA7<Analog>,
        pa15: PA15<Alternate<PushPull, 0>>,
        pb0: PB0<Analog>,
        pb1: PB1<Analog>,
        pb2: PB2<Analog>,
        pc3: PC3<Analog>,
        pc4: PC4<Analog>,
        pc5: PC5<Analog>,
        gpioa_moder: &mut MODER<'A'>,
        gpioa_otyper: &mut OTYPER<'A'>,
        gpiob_moder: &mut MODER<'B'>,
        gpiob_otyper: &mut OTYPER<'B'>,
        gpioc_moder: &mut MODER<'C'>,
        gpioc_otyper: &mut OTYPER<'C'>,
        clocks: Clocks,
    ) -> Self {
        // Use .into_push_pull_output_in_state(…) to set an initial state on pins

        // set the pins to very high speed mode, set the initial state, set the pin to output
        let sb = pc5.into_push_pull_output_in_state(gpioc_moder, gpioc_otyper, PinState::High).set_speed(Speed::VeryHigh);
        let lat = pc4.into_push_pull_output_in_state(gpioc_moder, gpioc_otyper, PinState::High).set_speed(Speed::VeryHigh);
        let rst = pc3.into_push_pull_output(gpioc_moder, gpioc_otyper).set_speed(Speed::VeryHigh);
        let sck = pb1.into_push_pull_output(gpiob_moder, gpiob_otyper).set_speed(Speed::VeryHigh);
        let sda = pa4.into_push_pull_output(gpioa_moder, gpioa_otyper).set_speed(Speed::VeryHigh);
        let c0 = pb2.into_push_pull_output(gpiob_moder, gpiob_otyper).set_speed(Speed::VeryHigh);
        let c1 = pa15.into_push_pull_output(gpioa_moder, gpioa_otyper).set_speed(Speed::VeryHigh);
        let c2 = pa2.into_push_pull_output(gpioa_moder, gpioa_otyper).set_speed(Speed::VeryHigh);
        let c3 = pa7.into_push_pull_output(gpioa_moder, gpioa_otyper).set_speed(Speed::VeryHigh);
        let c4 = pa6.into_push_pull_output(gpioa_moder, gpioa_otyper).set_speed(Speed::VeryHigh);
        let c5 = pa5.into_push_pull_output(gpioa_moder, gpioa_otyper).set_speed(Speed::VeryHigh);
        let c6 = pb0.into_push_pull_output(gpiob_moder, gpiob_otyper).set_speed(Speed::VeryHigh);
        let c7 = pa3.into_push_pull_output(gpioa_moder, gpioa_otyper).set_speed(Speed::VeryHigh);

        // create matrix
        let mut matrix = Matrix { sb, lat, rst, sck, sda, c0, c1, c2, c3, c4, c5, c6, c7 };
        
        // wait and then set rst high
        let mut delay = Delay::new(clocks);
        delay.delay_ms(100u16);
        matrix.rst.set_high();

        // initialize bank0
        matrix.init_bank0();
        matrix
    }

    /// Make a brief high pulse of the SCK pin
    fn pulse_sck(&mut self) {
        self.sck.set_high();
        self.sck.set_low();
    }

    /// Make a brief low pulse of the LAT pin
    fn pulse_lat(&mut self) {
        self.lat.set_low();
        self.lat.set_high();
    }

    /// Set the given row output in the chosen state
    fn row(&mut self, row: usize, state: PinState) {
        match row {
            0 => self.c0.set_state(state),
            1 => self.c1.set_state(state),
            2 => self.c2.set_state(state),
            3 => self.c3.set_state(state),
            4 => self.c4.set_state(state),
            5 => self.c5.set_state(state),
            6 => self.c6.set_state(state),
            7 => self.c7.set_state(state),
            _ => unreachable!(),
        }
    }

    /// Send a byte on SDA starting with the MSB and pulse SCK high after each bit
    fn send_byte(&mut self, pixel: u8) {
        //defmt::println!("Pixel: {}", pixel);
        for i in 0..8 {
            self.sda.set_state(((pixel >> (7 - i)) & 1 == 1).into());
            self.pulse_sck();
        }
    }

    /// Send a full row of bytes in BGR order and pulse LAT low. Gamma correction
    /// must be applied to every pixel before sending them. The previous row must
    /// be deactivated and the new one activated.
    pub fn send_row(&mut self, row: usize, pixels: &[Color]) {

        // Send the pixels
        for i in 0..8 {
            let pixel = pixels[7-i].gamma_correct();
            self.send_byte(pixel.b);
            if i == 6 {
                // Deactivate the previous row
                let previous_row = if row == 0 { 7 } else { row - 1 };
                self.row(previous_row, PinState::Low);
            }
            self.send_byte(pixel.g);
            self.send_byte(pixel.r);
        }

        // Activate the new row
        self.row(row, PinState::High);

        // Pulse LAT low
        self.pulse_lat();
    }

    /// Initialize bank0 by temporarily setting SB to low and sending 144 one bits,
    /// pulsing SCK high after each bit and pulsing LAT low at the end. SB is then
    /// restored to high.
    fn init_bank0(&mut self) {
        self.sb.set_low();
        for _ in 0..18 {
            self.send_byte(0b1111_1111);
        }
        self.pulse_lat();
        self.sb.set_high();
    }

    /// Display a full image, row by row, as fast as possible.
    pub fn display_image(&mut self, image: &Image) -> ! {
        // Do not forget that image.row(n) gives access to the content of row n,
        // and that self.send_row() uses the same format.
        loop {
            for row in 0..8 {
                self.send_row(row, image.row(row));
            }
        }
    }
}