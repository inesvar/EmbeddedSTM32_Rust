#![no_std]
#![no_main]

use stm32l4xx_hal::{pac, prelude::*};
use panic_probe as _;
use defmt_rtt as _;

pub use tp_led_matrix::image::*;
use tp_led_matrix::matrix::Matrix;

#[rtic::app(device = pac, dispatchers = [USART2, USART3])]
mod app {

    use super::*;
    use dwt_systick_monotonic::DwtSystick;
    use dwt_systick_monotonic::ExtU32;

    #[shared]
    struct Shared {
        image: Image,
    }

    #[local]
    struct Local {
        matrix: Matrix,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("defmt correctly initialized");
    
        let mut cp = cx.core;
        let dp = cx.device;

        // Create an instance of the timer
        let mut mono = DwtSystick::new(&mut cp.DCB, cp.DWT, cp.SYST, 80_000_000);

        // Initialize the clocks, hardware and matrix using your existing code
        let matrix = run(dp);
        let image :Image = Image::default();
        
        // Launch the display task
        display::spawn(mono.now()).unwrap();
        rotate_image::spawn(mono.now(), 0).unwrap();
    
        // Return the resources and the monotonic timer
        (Shared { image }, Local { matrix }, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {}
    }

    #[task(local = [matrix, next_line: usize = 0], shared = [image], priority = 2)]
    fn display(mut cx: display::Context, instant: Instant) {
        // Display line next_line (cx.local.next_line) of
        // the image (cx.local.image) on the matrix (cx.local.matrix).
        // All those are mutable references.
        cx.shared.image.lock(|image| {
            // Here you can use image, which is a &mut Image,
            // to display the appropriate row
            cx.local.matrix.send_row(*cx.local.next_line, image.row(*cx.local.next_line));
        });
        // Increment next_line up to 7 and wraparound to 0
        *cx.local.next_line = (*cx.local.next_line + 1)%8;
        // Spawn the display of the next row
        let next_display :Instant = instant + 1.secs() / 480;
        display::spawn_at(next_display, next_display).unwrap();
    }

    #[task(shared = [image])]
    fn rotate_image(mut cx: rotate_image::Context, instant: Instant, parameter: usize) {
        cx.shared.image.lock(|image| {
            match parameter {
                0 => *image = Image::gradient(RED).gamma_correct(),
                1 => *image = Image::gradient(GREEN).gamma_correct(),
                2 => *image = Image::gradient(BLUE).gamma_correct(),
                _ => unreachable!(),
            }
        });
        // Spawn the display of the next row
        let next_display :Instant = instant + 1.secs();
        let next_param = (parameter + 1)%3;
        rotate_image::spawn_at(next_display, next_display, next_param).unwrap();
    }

    #[monotonic(binds = SysTick, default = true)]
    type MyMonotonic = DwtSystick<80_000_000>;
    type Instant = <MyMonotonic as rtic::Monotonic>::Instant;

    fn run(dp: pac::Peripherals) -> Matrix {
        // Get high-level representations of hardware modules
        let mut rcc = dp.RCC.constrain();
        let mut flash = dp.FLASH.constrain();
        let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

        // Setup the clocks at 80MHz using HSI (by default since HSE/MSI are not configured).
        // The flash wait states will be configured accordingly.
        let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);
        
        // Split the GPIOs into individual pins
        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
        let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
        let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);

        // Construct the matrix
        let matrix = Matrix::new(
            gpioa.pa2,
            gpioa.pa3,
            gpioa.pa4,
            gpioa.pa5,
            gpioa.pa6,
            gpioa.pa7,
            gpioa.pa15,
            gpiob.pb0,
            gpiob.pb1,
            gpiob.pb2,
            gpioc.pc3,
            gpioc.pc4,
            gpioc.pc5,
            &mut gpioa.moder,
            &mut gpioa.otyper,
            &mut gpiob.moder,
            &mut gpiob.otyper,
            &mut gpioc.moder,
            &mut gpioc.otyper,
            clocks,
        );

        matrix
    }
}