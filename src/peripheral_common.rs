#![no_std]

#[derive(Debug)]
pub enum PeripheralError {
    InitializationFailed,
    ReadError,
    WriteError,
    UnsupportedOperation,
}

pub trait Peripheral {
    fn initialize(&mut self) -> Result<(), PeripheralError>;
}

// Optional trait for peripherals that can be explicitly shutdown or deactivated
pub trait Shutdown {
    fn shutdown(&mut self) -> Result<(), PeripheralError>;
}

// Trait for peripherals capable of reading data (generic)
pub trait Readable {
    type Output;
    fn read(&self) -> Result<Self::Output, PeripheralError>;
}

// Trait for peripherals capable of writing data (generic)
pub trait Writable {
    type Input;
    fn write(&mut self, data: Self::Input) -> Result<(), PeripheralError>;
}

// Specialized trait for temperature sensing peripherals
pub trait TemperatureSensor: Peripheral {
    // Reads the temperature in degrees Celsius
    fn read_temperature(&self) -> Result<f32, PeripheralError>;
}

// Specialized trait for humidity sensing peripherals
pub trait HumiditySensor: Peripheral {
    // Reads the humidity level as a percentage
    fn read_humidity(&self) -> Result<f32, PeripheralError>;
}

// Specialized trait for pressure sensing peripherals
pub trait PressureSensor: Peripheral {
    // Reads the atmospheric pressure in hPa (hectopascals)
    fn read_pressure(&self) -> Result<f32, PeripheralError>;
}