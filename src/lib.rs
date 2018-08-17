#![no_std]

extern crate embedded_hal as hal;
use hal::blocking::delay::DelayMs;
use hal::blocking::i2c::{Read, Write, WriteRead};

const I2C_ADDRESS: u8 = 0x40;
const HTU21_RESET: u8 = 0xFE;
const HTU21_START_TEMP: u8 = 0xE3;
const HTU21_START_HUM: u8 = 0xE5;

#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error
    I2c(E),
}

/// Driver for the HTU21F
#[derive(Debug, Default)]
pub struct HTU21D<I2C, D> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The concrete Delay implementation.
    delay: D,
}


impl<I2C, D, E> HTU21D<I2C, D>
    where
        I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
        D: DelayMs<u8>,
{
    /// Initialize the HTU21F driver.
    pub fn new(i2c: I2C, delay: D) -> Self {
        HTU21D {
            i2c,
            delay,
        }
    }

    pub fn reset(&mut self) -> Result<(), Error<E>> {
        let buf = [HTU21_RESET];
        self.i2c.write(I2C_ADDRESS, &buf).map_err(Error::I2c)?;
        self.delay.delay_ms(15);
        Ok(())

    }

    pub fn read_temperature(&mut self) -> Result<f32, Error<E>> {
        let buf = [HTU21_START_TEMP];
        self.i2c.write(I2C_ADDRESS, &buf).map_err(Error::I2c)?;
        self.delay.delay_ms(50);

        let mut buf = [0 as u8, 0, 0];
        self.i2c.read(I2C_ADDRESS, &mut buf).map_err(Error::I2c)?;

        let raw_temp: u16 = ((buf[0] as u16) << 8) + ((buf[1] as u16) << 0) ;

        let mut temp:f32 = raw_temp as f32;
        temp = temp * 175.72f32;
        temp = temp / 65536f32;
        temp = temp - 46.85f32;

        Ok(temp)

    }

    pub fn read_humidity(&mut self) -> Result<f32, Error<E>> {

        let buf = [HTU21_START_HUM];
        self.i2c.write(I2C_ADDRESS, &buf).map_err(Error::I2c)?;
        self.delay.delay_ms(50);

        let mut buf = [0 as u8, 0, 0];
        self.i2c.read(I2C_ADDRESS, &mut buf).map_err(Error::I2c)?;

        let raw_hum: u16 = ((buf[0] as u16) << 8) + ((buf[1] as u16) << 0) ;

        let mut humidity: f32 = raw_hum as f32;
        humidity = humidity * 125f32;
        humidity = humidity / 65536f32;
        humidity = humidity - 6f32;

        Ok(humidity)

    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
