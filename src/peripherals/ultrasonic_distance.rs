/// Support for ultra-sonic distance sensor
const SOUND_SPEED: f32 = 331.3;
const SOUND_SPEED_INC_OVER_TEMP: f32 = 0.606;

use embedded_hal::{
    blocking::delay::DelayUs,
    digital::v2::{InputPin, OutputPin},
};
use esp_hal::{systimer::SystemTimer, Delay};

pub struct USDistanceSensor<TriggerPin, EchoPin>
where
    TriggerPin: OutputPin<Error = core::convert::Infallible>,
    EchoPin: InputPin<Error = core::convert::Infallible>,
{
    trigger: TriggerPin,
    echo: EchoPin,
    delay: Delay,
}

impl<TriggerPin, EchoPin> USDistanceSensor<TriggerPin, EchoPin>
where
    TriggerPin: OutputPin<Error = core::convert::Infallible>,
    EchoPin: InputPin<Error = core::convert::Infallible>,
{
    // Method to initialize the sensor
    pub fn new(mut trigger: TriggerPin, echo: EchoPin, delay: Delay) -> Self {
        trigger.set_low().unwrap();
        USDistanceSensor {
            trigger,
            echo,
            delay,
        }
    }

    pub fn measure(&mut self, ambient_temp: f32) -> f32 {
        let sound_speed = SOUND_SPEED + (SOUND_SPEED_INC_OVER_TEMP * ambient_temp);
        self.trigger.set_high().unwrap();
        self.delay.delay_us(10 as u32);
        self.trigger.set_low().unwrap();

        while self.echo.is_low().unwrap() {}
        let start_timestamp = SystemTimer::now();
        while self.echo.is_high().unwrap() {}
        let end_timestamp = SystemTimer::now();

        sound_speed * ((end_timestamp as f32 - start_timestamp as f32) / 10000.0) / 2.0
    }
}
