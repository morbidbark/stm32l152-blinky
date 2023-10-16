use stm32l1::stm32l151 as pac;

pub struct Button<'a> {
    periphs: &'a pac::Peripherals,
}
impl<'a> Button<'a> {
    pub fn init(periphs: &'a pac::Peripherals) -> Self {
        Self {
            periphs,
        }
    }
    
    pub fn is_pressed(&self) -> bool {
        let gpioa = &self.periphs.GPIOA;
        gpioa.idr.read().idr0().bit()
    }
}
