#![no_main]
#![no_std]

pub const RED: u8 = 0;
pub const GREEN: u8 = 1;
pub const BLUE: u8 = 2;

// use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use embedded_hal::digital::PinState;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{
    board::Board,
    // hal::gpio,
    hal::gpio::Level,
    hal::gpio::OpenDrainConfig,
    hal::timer::Timer,
    // hal::spi,
    hal::uarte::{Baudrate, Parity, Uarte},
};

fn serial_write<T>(serial: &mut Uarte<T>, buffer: &[u8]) -> ()
where
    T: microbit::hal::uarte::Instance,
{
    // for each byte in the buffer, write it out to the serial port
    for b in buffer {
        match serial.write(&[*b]) {
            Ok(_r) => (),
            Err(e) => rprintln!("Serial Error: {:?}", e),
        }
    }
}

#[cortex_m_rt::entry]
fn start_here() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut led_timer: Timer<microbit::pac::TIMER1> = Timer::new(board.TIMER1);

    // Set up UARTE for microbit v2 using UartePort wrapper
    let mut serial = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );

    // SPI Initialization
    // let mut mosi = microbit::gpio::MOSI; //.into_push_pull_input(Level::Low);
    // let mut miso = gpio::miso.into_push_pull_input(Level::Low);
    // let mut sck  = gpio::sck.into_push_pull_output(Level::Low);
    // let mut cs   = board.edge.e16.into_push_pull_output(Level::High);

    // let mut spi = spi::Spi::new(

    // );

    // Clear terminal
    serial_write(&mut serial, b"\x1Bc");

    // Setup the low side of the matrix LED
    board.display_pins.col5.set_state(PinState::Low).unwrap();

    // Setup the RGB LED pins as open drain with inital setting LOW
    // Turns off the RGB LED
    let mut red_left: microbit::hal::gpio::p0::P0_02<
        microbit::hal::gpio::Output<microbit::hal::gpio::OpenDrain>,
    > = board
        .edge
        .e00
        .into_open_drain_output(OpenDrainConfig::HighDrive0Disconnect1, Level::Low);
    let mut green_left: microbit::hal::gpio::p0::P0_03<
        microbit::hal::gpio::Output<microbit::hal::gpio::OpenDrain>,
    > = board
        .edge
        .e01
        .into_open_drain_output(OpenDrainConfig::HighDrive0Disconnect1, Level::Low);
    let mut blue_left: microbit::hal::gpio::p0::P0_04<
        microbit::hal::gpio::Output<microbit::hal::gpio::OpenDrain>,
    > = board
        .edge
        .e02
        .into_open_drain_output(OpenDrainConfig::HighDrive0Disconnect1, Level::Low);

    // Initial color state

    loop {
        // Toggle the Matrix LED on
        board.display_pins.row3.set_state(PinState::High).unwrap();

        turn_on(
            0,
            100,
            20,
            500,
            &mut red_left,
            &mut green_left,
            &mut blue_left,
            &mut led_timer,
        );

        turn_on(
            0,
            20,
            100,
            500,
            &mut red_left,
            &mut green_left,
            &mut blue_left,
            &mut led_timer,
        );

        turn_on(
            100,
            0,
            50,
            500,
            &mut red_left,
            &mut green_left,
            &mut blue_left,
            &mut led_timer,
        );

        turn_on(
            100,
            20,
            20,
            500,
            &mut red_left,
            &mut green_left,
            &mut blue_left,
            &mut led_timer,
        );

        turn_on(
            50,
            100,
            0,
            500,
            &mut red_left,
            &mut green_left,
            &mut blue_left,
            &mut led_timer,
        );
        // On time
        timer.delay_ms(500);

        // Toggle the Matrix LED off
        board.display_pins.row3.set_state(PinState::Low).unwrap();

        // depending on color state, turn off the RGB LED
        // Off time
        timer.delay_ms(500);
    }
}

fn turn_on(
    red_percent: u32,
    green_percent: u32,
    blue_percent: u32,
    total_time_ms: u32,
    red: &mut microbit::hal::gpio::p0::P0_02<
        microbit::hal::gpio::Output<microbit::hal::gpio::OpenDrain>,
    >,
    green: &mut microbit::hal::gpio::p0::P0_03<
        microbit::hal::gpio::Output<microbit::hal::gpio::OpenDrain>,
    >,
    blue: &mut microbit::hal::gpio::p0::P0_04<
        microbit::hal::gpio::Output<microbit::hal::gpio::OpenDrain>,
    >,
    timer: &mut Timer<microbit::pac::TIMER1>,
) {
    let mut total_time_elapsed = 0;
    let mut cycle = 0;
    while total_time_elapsed < total_time_ms * 1000 {
        if cycle < red_percent {
            red.set_high().unwrap();
        } else {
            red.set_low().unwrap();
        }
        if cycle < green_percent {
            green.set_high().unwrap();
        } else {
            green.set_low().unwrap();
        }
        if cycle < blue_percent {
            blue.set_high().unwrap();
        } else {
            blue.set_low().unwrap();
        }

        timer.delay_us(10);
        total_time_elapsed += 10;
        cycle += 1;
        if cycle > 100 {
            cycle = 0;
        }
    }

    red.set_low().unwrap();
    green.set_low().unwrap();
    blue.set_low().unwrap();
}
