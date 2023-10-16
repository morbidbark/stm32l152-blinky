use stm32l1::stm32l151 as pac;

pub struct LED<'a> {
    active: bool,
    periphs: &'a pac::Peripherals,
}
impl<'a> LED<'a> {
    pub fn init(periphs: &'a pac::Peripherals) -> Self {
        let gpiob = &periphs.GPIOB;
        // set general purpose output
        gpiob.moder.write(|w| {
            w.moder6().bits(0b01)
        });
        // set push-pull output
        gpiob.otyper.write(|w| {
            w.ot6().bit(false)
        });
        // set low speed output
        gpiob.ospeedr.write(|w| {
            w.ospeedr6().bits(0b00)
        });
        // set NO push-up/pull-down resistor
        gpiob.pupdr.write(|w| unsafe {
            w.pupdr6().bits(0b00)
        });
        
        let mut led = Self {
            active: false,
            periphs,
        };
        led.set_low();
        led
    }

    pub fn set_high(&mut self) {
        let gpiob = &self.periphs.GPIOB;
        gpiob.bsrr.write(|w| {
            w.bs6().set_bit()
        });
        self.active = true;
    }

    pub fn set_low(&mut self) {
        let gpiob = &self.periphs.GPIOB;
        gpiob.bsrr.write(|w| {
            w.br6().set_bit()
        });
        self.active = false;
    }
    pub fn toggle(&mut self) {
        if self.active {
            self.set_low();
        } else {
            self.set_high();
        }
    }
}
