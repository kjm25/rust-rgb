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
    hal::pwm::Channel,
    hal::pwm,
    hal::gpio::OpenDrainConfig,
    hal::timer::Timer,
    // hal::spi,
    hal::uarte::{Baudrate, Parity, Uarte},
};


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


    // Setup the low side of the matrix LED
    board.display_pins.col5.set_state(PinState::Low).unwrap();

    // Setup the RGB LED pins as open drain with inital setting LOW
    // Turns off the RGB LED
    let mut red_left= board
        .edge
        .e00
        .into_push_pull_output(Level::Low);
    let green_left = board
        .edge
        .e01
        .into_push_pull_output(Level::Low);
    let mut blue_left= board
        .edge
        .e02
        .into_push_pull_output(Level::Low);

    // Initial color state

    let mut green_pwn = pwm::Pwm::new(board.PWM0);
    green_pwn.set_output_pin(Channel::C0, green_left.degrade());

    let mut red_pwm = pwm::Pwm::new(board.PWM1);
    green_pwn.set_output_pin(Channel::C1, red_left.degrade());

    let mut blue_pwm = pwm::Pwm::new(board.PWM2);
    blue_pwm.set_output_pin(Channel::C1, blue_left.degrade());

    loop {
        // Toggle the Matrix LED on
        board.display_pins.row3.set_state(PinState::High).unwrap();

        turn_on(
            0,
            100,
            0,
            500,
            &mut red_pwm,
            &mut green_pwn,
            &mut blue_pwm,
            &mut led_timer,
        );

        turn_on(
            100,
            0,
            100,
            500,
            &mut red_pwm,
            &mut green_pwn,
            &mut blue_pwm,
            &mut led_timer,
        );

        turn_on(
            100,
            100,
            100,
            500,
            &mut red_pwm,
            &mut green_pwn,
            &mut blue_pwm,
            &mut led_timer,
        );

        turn_on(
            0,
            0,
            0,
            500,
            &mut red_pwm,
            &mut green_pwn,
            &mut blue_pwm,
            &mut led_timer,
        );

        // On time
        

        // Toggle the Matrix LED off
        board.display_pins.row3.set_state(PinState::Low).unwrap();

        // depending on color state, turn off the RGB LED
        // Off time
    }
}

fn turn_on(
    red_percent: u16,
    green_percent: u16,
    blue_percent: u16,
    total_time_ms: u32,
    red: &mut pwm::Pwm<microbit::pac::PWM1>,
    green: &mut pwm::Pwm<microbit::pac::PWM0>,
    blue: &mut pwm::Pwm<microbit::pac::PWM2>,
    timer: &mut Timer<microbit::pac::TIMER1>,
) {
    
    green.set_max_duty(100).set_duty_off(Channel::C0, green_percent);
    green.enable();
    green.loop_inf();

    red.set_max_duty(100).set_duty_off(Channel::C1, red_percent);
    red.enable();
    red.loop_inf();

    blue.set_max_duty(100).set_duty_off(Channel::C1, blue_percent);
    blue.enable();
    blue.loop_inf();


    timer.delay_ms(total_time_ms);

    green.disable();
    red.disable();
    blue.disable();
}
