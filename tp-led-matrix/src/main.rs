#![no_std]
#![no_main]

use stm32l4xx_hal::{pac, pac::USART1, prelude::*};
use stm32l4xx_hal::serial::{Config, Event, Rx, Serial};
use panic_probe as _;
use defmt_rtt as _;


pub use tp_led_matrix::image::*;
use tp_led_matrix::matrix::Matrix;
use core::mem::{swap, MaybeUninit};
use heapless::pool::{Box, Node, Pool};

#[rtic::app(device = pac, dispatchers = [USART2, USART3])]
mod app {

    use super::*;
    use dwt_systick_monotonic::DwtSystick;
    use dwt_systick_monotonic::ExtU32;

    #[shared]
    struct Shared {
        pool: Pool<Image>,
        next_image: Option<Box<Image>>,
    }

    #[local]
    struct Local {
        matrix: Matrix,
        usart1_rx: Rx<USART1>,
        current_image: Box<Image>,
        rx_image: Box<Image>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("defmt correctly initialized");
    
        let mut cp = cx.core;
        let dp = cx.device;

        // Create an instance of the timer
        let mut mono = DwtSystick::new(&mut cp.DCB, cp.DWT, cp.SYST, 80_000_000);

        // Initialize the clocks, hardware and matrix using your existing code
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
        let pool: Pool<Image> = Pool::new();
        unsafe {
            static mut MEMORY: MaybeUninit<[Node<Image>; 3]> = MaybeUninit::uninit();
            pool.grow_exact(&mut MEMORY);   // static mut access is unsafe
        }
        let current_image = pool.alloc().unwrap().init(Image::default());
        let rx_image = pool.alloc().unwrap().init(Image::default());
        let next_image: Option<Box<Image>> = None;

        // Configure the serial port
        let tx = gpiob.pb6.into_alternate::<7>(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
        let rx = gpiob.pb7.into_alternate::<7>(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

        let usart1_config = Config::from(38_400.bps());
        let mut serial = Serial::usart1(dp.USART1, (tx, rx), usart1_config, clocks, &mut rcc.apb2);
        serial.listen(Event::Rxne);
        let usart1_rx = serial.split().1;
        
        // Launch the display task
        display::spawn(mono.now()).unwrap();
    
        // Return the resources and the monotonic timer
        (Shared { pool, next_image }, Local { matrix, usart1_rx, current_image, rx_image }, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {}
    }

    #[task(local = [matrix, current_image, next_line: usize = 0], shared = [ next_image, pool ], priority = 2)]
    fn display(mut cx: display::Context, instant: Instant) {
        // if the display of the led matrix starts over
        if *cx.local.next_line == 0 {
            // and a full new image is available
            cx.shared.next_image.lock( |next_image : &mut Option<Box<Image>> |
            if let Some(mut image) = next_image.take() {
                // replace the current_image with next_image
                swap(&mut image, cx.local.current_image);
                // discard the old current_image
                cx.shared.pool.lock( |pool : &mut Pool<Image> |
                    pool.free(image))
            });
        }
        // display a row
        cx.local.matrix.send_row(*cx.local.next_line, cx.local.current_image.row(*cx.local.next_line));
        // Increment next_line up to 7 and wraparound to 0
        *cx.local.next_line = (*cx.local.next_line + 1)%8;
        // Spawn the display of the next row
        let next_display :Instant = instant + 1.secs() / 480;
        display::spawn_at(next_display, next_display).unwrap();
    }

    #[task(binds = USART1, local = [usart1_rx, rx_image, next_pos: usize = 0], shared = [ pool, next_image])]
    fn receive_byte(mut cx: receive_byte::Context) {
        let next_pos: &mut usize = cx.local.next_pos;
        let rx_image: &mut Box<Image> = cx.local.rx_image;
        if let Ok(b) = cx.local.usart1_rx.read() {
            // Handle the incoming byte according to the SE203 protocol
            // and update next_image
            // Do not forget that next_image.as_mut() might be handy here!
            if b == 0xFF {
                *next_pos = 0;
                return;
            }
            if *next_pos == 8 * 8 * 3 {
                return;
            }
            rx_image.as_mut()[*next_pos] = b;
            *next_pos += 1;
            // If the received image is complete, make it available to
            // the display task.
            if *next_pos == 8 * 8 * 3 {
                cx.shared.next_image.lock(|next_image: &mut Option<Box<Image>>| {
                    // if some image was already ready to be displayed
                    if let Some(image) = next_image.take() {
                        // discard it
                        cx.shared.pool.lock(|pool: &mut Pool<Image>| {
                            pool.free(image);
                        })
                    }
                    // Obtain a new future_image from the pool and swap it with rx_image
                    cx.shared.pool.lock(|pool: &mut Pool<Image>| {
                        let mut future_image: Box<Image> = pool.alloc().unwrap().init(Image::default());
                        swap(&mut future_image, rx_image);
                        next_image.replace(future_image);
                    });
                });
            }
        }
    }

    #[monotonic(binds = SysTick, default = true)]
    type MyMonotonic = DwtSystick<80_000_000>;
    type Instant = <MyMonotonic as rtic::Monotonic>::Instant;
}