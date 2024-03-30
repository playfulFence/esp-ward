use embedded_hal::digital::v2::InputPin;

/// Minimalistic abstraction layer over PIR (motion)
pub struct PIRSensor<PIN: InputPin> {
    inner: PIN,
}

impl<PIN: InputPin<Error = core::convert::Infallible>> PIRSensor<PIN> {
    pub fn new(pin: PIN) -> Self {
        PIRSensor { inner: pin }
    }

    pub fn motion_detected(&mut self) -> bool {
        self.inner.is_high().unwrap()
    }
}
