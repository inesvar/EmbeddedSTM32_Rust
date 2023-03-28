#![no_std]
#![no_main]

use stm32l4xx_hal::{pac, prelude::*};
use panic_probe as _;
use defmt_rtt as _;

pub use tp_led_matrix::image::*;
use tp_led_matrix::matrix::Matrix;

#[rtic::app(device = pac)]
mod app {

    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        matrix: Matrix
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("defmt correctly initialized");
    
        let _cp = cx.core;
        let dp = cx.device;
    
        // Initialize the clocks, hardware and matrix using your existing code
        let matrix = run(_cp, dp);
    
        // Return the resources and the monotonic timer
        (Shared {}, Local { matrix }, init::Monotonics())
    }

    #[idle(local = [matrix])]
    fn idle(cx: idle::Context) -> ! {
        cx.local.matrix.display_image(&Image::gradient(BLUE).gamma_correct())
    }

    fn run(_cp: pac::CorePeripherals, dp: pac::Peripherals) -> Matrix {
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