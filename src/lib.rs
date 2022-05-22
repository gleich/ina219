#![no_std]

use embedded_hal::blocking::i2c::Write;

/// A struct to outline data related to the INA219 TI IC
pub struct INA219<I2C> {
    pub i2c: I2C,
    pub address: u8,
}

pub enum Error<I2CError> {
    I2cError(I2CError),
}

impl<I2C, I2CError> INA219<I2C> where I2C: Write<Error = I2CError> {}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2cError(error)
    }
}

pub mod addresses {
    pub const CONFIG: u8 = 0x00;
    pub const SHUNTVOLTAGE: u8 = 0x01;
    pub const BUSVOLTAGE: u8 = 0x02;
    pub const POWER: u8 = 0x03;
    pub const CURRENT: u8 = 0x04;
    pub const CALIBRATION: u8 = 0x05;

    pub mod voltage_range {
        pub const V16: u8 = 0x00;
        pub const V32: u8 = 0x01;
    }

    pub mod gain {
        pub const DIV_1_40MV: u8 = 0x00;
        pub const DIV_2_80MV: u8 = 0x01;
        pub const DIV_4_160MV: u8 = 0x02;
        pub const DIV_8_320V: u8 = 0x03;
    }

    pub mod adc_resolution {
        pub const ADCRES_9BIT_1S: u8 = 0x00;
        pub const ADCRES_10BIT_1S: u8 = 0x01;
        pub const ADCRES_11BIT_1S: u8 = 0x02;
        pub const ADCRES_12BIT_1S: u8 = 0x03;
        pub const ADCRES_12BIT_2S: u8 = 0x09;
        pub const ADCRES_12BIT_4S: u8 = 0x0A;
        pub const ADCRES_12BIT_8S: u8 = 0x0B;
        pub const ADCRES_12BIT_16S: u8 = 0x0C;
        pub const ADCRES_12BIT_32S: u8 = 0x0D;
        pub const ADCRES_12BIT_64S: u8 = 0x0E;
        pub const ADCRES_12BIT_128S: u8 = 0x0F;
    }

    pub mod mode {
        pub const POWERDOWN: u8 = 0x00;
        pub const SVOLT_TRIGGERED: u8 = 0x01;
        pub const BVOLT_TRIGGERED: u8 = 0x02;
        pub const SANDBVOLT_TRIGGERED: u8 = 0x03;
        pub const ADCOFF: u8 = 0x04;
        pub const SVOLT_CONTINUOUS: u8 = 0x05;
        pub const BVOLT_CONTINUOUS: u8 = 0x06;
        pub const SANDBVOLT_CONTINUOUS: u8 = 0x07;
    }
}
