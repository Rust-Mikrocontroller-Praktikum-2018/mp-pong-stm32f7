use super::{LAYER_1_START, LAYER_1_START_2, Lcd};
use board::ltdc::Ltdc;
use board::ltdc::L1clutwr;
use board::rcc::Rcc;
use embedded::interfaces::gpio::{Gpio, OutputPin};

const HEIGHT: u16 = super::HEIGHT as u16;
const WIDTH: u16 = super::WIDTH as u16;
const LAYER_1_OCTETS_PER_PIXEL: u16 = super::LAYER_1_OCTETS_PER_PIXEL as u16;

pub fn init(ltdc: &'static mut Ltdc, rcc: &mut Rcc, gpio: &mut Gpio) -> Lcd {
    // init gpio pins
    let (mut display_enable, mut backlight_enable) = init_pins(gpio);

    // enable LTDC and DMA2D clocks
    rcc.ahb1enr.update(|r| r.set_dma2den(true));
    rcc.apb2enr.update(|r| r.set_ltdcen(true));

    // disable LTDC
    ltdc.gcr.update(|r| r.set_ltdcen(false));

    // disable PLLSAI clock
    rcc.cr.update(|r| r.set_pllsaion(false));
    while rcc.cr.read().pllsairdy() {}

    rcc.pllsaicfgr.update(|r| {
        r.set_pllsain(192);
        r.set_pllsair(5);
    });

    // set division factor for LCD_CLK
    rcc.dkcfgr1.update(|r| {
        r.set_pllsaidivr(0b01 /* = 4 */)
    });

    // enable PLLSAI clock
    rcc.cr.update(|r| r.set_pllsaion(true));
    while !rcc.cr.read().pllsairdy() {}

    // configure the HS, VS, DE and PC polarity
    ltdc.gcr.update(|r| {
        r.set_pcpol(false);
        r.set_depol(false);
        r.set_hspol(false);
        r.set_vspol(false);
    });

    // set synchronization size
    ltdc.sscr.update(|r| {
        r.set_hsw(41 - 1); // horizontal_sync_width
        r.set_vsh(10 - 1); // vertical_sync_height
    });

    // set accumulated back porch
    ltdc.bpcr.update(|r| {
        r.set_ahbp(41 + 13 - 1); // accumulated_horizontal_back_porch
        r.set_avbp(10 + 2 - 1); // accumulated_vertical_back_porch
    });

    // set accumulated active width
    ltdc.awcr.update(|r| {
        r.set_aaw(WIDTH + 41 + 13 - 1); // accumulated_active_width
        r.set_aah(HEIGHT + 10 + 2 - 1); // accumulated_active_height
    });

    // set total width
    ltdc.twcr.update(|r| {
        r.set_totalw(WIDTH + 41 + 13 + 32 - 1); // total_width
        r.set_totalh(HEIGHT + 10 + 2 + 2 - 1); // total_height
    });

    // set background color
    ltdc.bccr.update(|r| r.set_bc(0x00_00ff)); // background_color blue

    // enable the transfer error interrupt and the FIFO underrun interrupt
    ltdc.ier.update(|r| {
        r.set_terrie(true); // TRANSFER_ERROR_INTERRUPT_ENABLE
        r.set_fuie(true); // FIFO_UNDERRUN_INTERRUPT_ENABLE
        r.set_lie(true); // LINE_INTERRUPT_ENABLE
    });

    // set the line the interrupt should happen on
    ltdc.lipcr.update(|r| {
        r.set_lipos(HEIGHT);
    });

    // configure layers

    // configure horizontal start and stop position
    ltdc.l1whpcr.update(|r| {
        r.set_whstpos(0 + 41 + 13); // window_horizontal_start_position
        r.set_whsppos(WIDTH + 41 + 13 - 1); // window_horizontal_stop_position
    });

    // configure vertical start and stop position
    ltdc.l1wvpcr.update(|r| {
        r.set_wvstpos(0 + 10 + 2); // window_vertical_start_position
        r.set_wvsppos(HEIGHT + 10 + 2 - 1); // window_vertical_stop_position
    });

    // specify pixed format
    ltdc.l1pfcr.update(|r| r.set_pf(0b111)); // set_pixel_format to L8

    // configure default color values
    ltdc.l1dccr.update(|r| {
        r.set_dcalpha(255);
        r.set_dcred(0);
        r.set_dcgreen(255);
        r.set_dcblue(0);
    });

    // configure color frame buffer start address
    // ltdc.l1cfbar.update(|r| r.set_cfbadd(LAYER_1_START as u32)); // don't draw in waiting time

    // configure color frame buffer line length and pitch
    ltdc.l1cfblr.update(|r| {
        r.set_cfbp(WIDTH * LAYER_1_OCTETS_PER_PIXEL); // pitch
        r.set_cfbll(WIDTH * LAYER_1_OCTETS_PER_PIXEL + 3); // line_length
    });

    // configure frame buffer line number
    ltdc.l1cfblnr.update(|r| r.set_cfblnbr(HEIGHT)); // line_number

    // define CLUT for layer 1
    for c in 0..=255 {
        let mut clut = L1clutwr::default();
        /*clut.set_red(if (c > 100) {0} else {255});
        clut.set_blue(if (c > 200) {0} else {255});
        clut.set_green(c);*/
        clut.set_red(c);
        clut.set_green(c);
        clut.set_blue(c);
        clut.set_clutadd(c);

        ltdc.l1clutwr.write(clut);
    }

    ltdc.l1cr.update(|r| {
        r.set_len(true); // enable layer 1
        //r.set_cluten(true); // enable CLUT for layer 1
    });

    // reload shadow registers
    ltdc.srcr.update(|r| r.set_imr(true)); // IMMEDIATE_RELOAD

    // enable LTDC
    ltdc.gcr.update(|r| r.set_ltdcen(true));

    // enable display and backlight
    display_enable.set(true);
    backlight_enable.set(true);

    Lcd {
        controller: ltdc,
        display_enable: display_enable,
        backlight_enable: backlight_enable,
        layer_1_in_use: false,
        write_to_buffer_2: false,
    }
}

pub fn init_pins(gpio: &mut Gpio) -> (OutputPin, OutputPin) {
    use embedded::interfaces::gpio::Pin::*;
    use embedded::interfaces::gpio::Port::*;
    use embedded::interfaces::gpio::{AlternateFunction, OutputSpeed, OutputType, Resistor};

    // Red
    let r0 = (PortI, Pin15);
    let r1 = (PortJ, Pin0);
    let r2 = (PortJ, Pin1);
    let r3 = (PortJ, Pin2);
    let r4 = (PortJ, Pin3);
    let r5 = (PortJ, Pin4);
    let r6 = (PortJ, Pin5);
    let r7 = (PortJ, Pin6);

    // Green
    let g0 = (PortJ, Pin7);
    let g1 = (PortJ, Pin8);
    let g2 = (PortJ, Pin9);
    let g3 = (PortJ, Pin10);
    let g4 = (PortJ, Pin11);
    let g5 = (PortK, Pin0);
    let g6 = (PortK, Pin1);
    let g7 = (PortK, Pin2);

    // Blue
    let b0 = (PortE, Pin4);
    let b1 = (PortJ, Pin13);
    let b2 = (PortJ, Pin14);
    let b3 = (PortJ, Pin15);
    let b4 = (PortG, Pin12);
    let b5 = (PortK, Pin4);
    let b6 = (PortK, Pin5);
    let b7 = (PortK, Pin6);

    let clk = (PortI, Pin14);
    let data_enable = (PortK, Pin7);
    let hsync = (PortI, Pin10);
    let vsync = (PortI, Pin9);

    let pins = [
        r0,
        r1,
        r2,
        r3,
        r4,
        r5,
        r6,
        r7,
        g0,
        g1,
        g2,
        g3,
        g4,
        g5,
        g6,
        g7,
        b0,
        b1,
        b2,
        b3,
        b4,
        b5,
        b6,
        b7,
        clk,
        data_enable,
        hsync,
        vsync,
    ];
    gpio.to_alternate_function_all(
        &pins,
        AlternateFunction::AF14,
        OutputType::PushPull,
        OutputSpeed::High,
        Resistor::NoPull,
    ).unwrap();

    // Display control
    let display_enable_pin = (PortI, Pin12);
    let backlight_pin = (PortK, Pin3);

    let display_enable = gpio.to_output(
        display_enable_pin,
        OutputType::PushPull,
        OutputSpeed::Low,
        Resistor::PullDown,
    ).unwrap();
    let backlight = gpio.to_output(
        backlight_pin,
        OutputType::PushPull,
        OutputSpeed::Low,
        Resistor::PullDown,
    ).unwrap();

    (display_enable, backlight)
}
