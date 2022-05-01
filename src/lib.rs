#![no_std]

/// A struct to outline data related to the INA219 TI IC
pub struct INA219<I2C> {
    pub i2c: I2C,
    pub address: u8,
}


pub enum Error<I2CError> {
    I2cError(I2CError),
}

impl<E> From <E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2cError(error)
    }
}
