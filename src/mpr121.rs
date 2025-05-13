#[cfg(feature = "sync")]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;
use strum::IntoEnumIterator;

use crate::{constants::*, registers::*, Channel, DebounceNumber};
use crate::{Mpr121Address, Mpr121Error};

/// This is the Sensor itself and takes in an I2C Device or bus see the examples folder for more details. The driver can work in either Async or Sync mode depending on which version of the embedded-hal you are using
pub struct Mpr121<I2C: I2c> {
    pub(crate) i2c: I2C,
    pub(crate) addr: Mpr121Address,
}

impl<I2C: I2c> Mpr121<I2C> {
    ///Creates the driver for the given I²C ports. Assumes that the I²C port is configured as master.
    ///
    /// If `use_auto_config` is set, the controller will use its auto configuration routine to setup
    /// charging parameters whenever it is transitioned from STOP to START mode.
    ///
    /// If `check_reset_flags` is set, the reset will be checked by reading back the 0x5C register. Note however, that
    /// sometime circuit configurations might be too slow/setup-incorrectly for that check. Which is why it is optional.
    ///
    /// Note that we use the same default values as the Adafruit implementation, except for threshold values.
    /// Use [set_thresholds](Self::set_thresholds) to define those.
    ///
    /// In the event of an error, returns [Mpr121Error]
    #[maybe_async::maybe_async]
    pub async fn new(
        i2c: I2C,
        addr: Mpr121Address,
        use_auto_config: bool,
        check_reset_flags: bool,
    ) -> Result<Self, Mpr121Error> {
        let mut dev = Mpr121 { i2c, addr };

        //TODO: Add Check to see if device is present on the bus This caught me out when refactoring and thus didnt realise it wasnt plugged in
        //reset
        let error = dev
            .write_register(Register::SoftReset, SOFT_RESET_VALUE)
            .await
            .err();
        error.map(|e| match e {
            Mpr121Error::ReadError(reg) => Mpr121Error::ResetFailed {
                was_read: true,
                reg,
            },
            Mpr121Error::WriteError(reg) => Mpr121Error::ResetFailed {
                was_read: false,
                reg,
            },
            _ => Mpr121Error::ResetFailed {
                was_read: false,
                reg: Register::SoftReset,
            },
        });

        // Stop
        dev.write_register(Register::Ecr, 0x0).await?;

        if check_reset_flags {
            // read config register
            let config = dev
                .read_reg8(Register::GlobalChargeDischargeTimeConfig)
                .await?;

            if config != Register::GlobalChargeDischargeTimeConfig.get_default_value() {
                return Err(Mpr121Error::InitFailed {
                    // Check if device is having a short circuit fault
                    over_current_protection: dev.is_over_current_set().await?,
                });
            }
        }
        //Initialise the device to the similar settings as Adafruit
        dev.set_thresholds(DEFAULT_TOUCH_THRESHOLD, DEFAULT_RELEASE_THRESOLD)
            .await?;
        dev.initialise_registers(use_auto_config).await?;

        Ok(dev)
    }

    #[maybe_async::maybe_async]
    async fn initialise_registers(&mut self, use_auto_config: bool) -> Result<(), Mpr121Error> {
        //Setup Filters MHD==MaximumHalfDelta, NHD=NoiseHalfDelta
        // Have a look at 5.5 in the data sheet for more information.

        self.write_register(Register::MaximumHalfDeltaRising, 0x01)
            .await?;
        self.write_register(Register::NoiseCountLimitRising, 0x01)
            .await?;
        self.write_register(Register::NoiseCountLimitRising, 0x0e)
            .await?;
        self.write_register(Register::FilterDelayCountLimitRising, 0x00)
            .await?;

        self.write_register(Register::MaximmHalfDeltaFalling, 0x01)
            .await?;
        self.write_register(Register::NoiseHalfDeltaFalling, 0x05)
            .await?;
        self.write_register(Register::NoiseCountLimitFalling, 0x01)
            .await?;
        self.write_register(Register::FilterDelayCountFalling, 0x00)
            .await?;

        self.write_register(Register::NoiseHalfDeltaTouched, 0x00)
            .await?;
        self.write_register(Register::NoiseCountLimitTouched, 0x00)
            .await?;
        self.write_register(Register::FilterDelayCountLimitTouched, 0x00)
            .await?;

        self.write_register(Register::Debounce, 0x0).await?;
        self.write_register(
            Register::GlobalChargeDischargeCurrentConfig,
            Register::GlobalChargeDischargeCurrentConfig.get_default_value(),
        )
        .await?;
        self.write_register(Register::GlobalChargeDischargeTimeConfig, 0x20)
            .await?;

        if use_auto_config {
            self.write_register(Register::AutoConfig0, 0x0b).await?;

            //Use 3.3V VDD
            self.write_register(Register::UpLimit, 200).await?; // = ((Vdd - 0.7)/Vdd) * 256;
            self.write_register(Register::TargetLimit, 180).await?; // = UPLIMIT * 0.9
            self.write_register(Register::LowLimit, 130).await?; // = UPLIMIT * 0.65
        }
        //enable electrodes and return to start mode
        let ecr_setting = 0b10000000
            + u8::try_from(Channel::iter().len())
                .expect("This should not fail, number of channels should be able to fit in 8 bits"); // enable all 12 electrodes
        self.write_register(Register::Ecr, ecr_setting).await?;
        Ok(())
    }

    /// Initializes the driver assuming the sensors address is the default one (0x5a).
    /// If this fails, consider searching for the driver.
    /// Or following the documentation on setting a driver address, and use [new](Self::new) to specify the address.
    ///
    /// Have a look at [new](Self::new) for further documentation.
    #[maybe_async::maybe_async]
    pub async fn new_default(i2c: I2C) -> Result<Self, Mpr121Error> {
        let result = Self::new(i2c, Mpr121Address::Default, false, true).await?;
        Ok(result)
    }
    /// Returns true if over-current is detected by the device.
    /// In that case you probably have to check your circuit
    ///
    /// In the event of an error [Mpr121Error] is returned
    #[maybe_async::maybe_async]
    pub async fn is_over_current_set(&mut self) -> Result<bool, Mpr121Error> {
        const OVER_CURRENT_PROTECTION_FLAG_MASK: u8 = 0b1 << 7;
        let read = self.read_reg8(Register::TouchStatus8_11).await?;
        //If bit D7 is set, we have OVCF
        Ok((read & (OVER_CURRENT_PROTECTION_FLAG_MASK)) > 0)
    }

    /// Set the touch and release threshold for all channels. Usually the touch threshold is a little bigger than the release
    /// threshold. This creates some debounce characteristics. The correct thresholds depend on the application.
    ///
    /// Have a look at [note AN3892](https://www.nxp.com/docs/en/application-note/AN3892.pdf) of the mpr121 guidelines.
    /// In the event of an error [Mpr121Error] is returned
    #[maybe_async::maybe_async]
    pub async fn set_thresholds(&mut self, touch: u8, release: u8) -> Result<(), Mpr121Error> {
        for channel in Channel::iter() {
            //Note ignoring false set thresholds
            self.write_register(Register::get_treshold_register(channel), touch)
                .await?;
            self.write_register(Register::get_release_register(channel), release)
                .await?;
        }
        Ok(())
    }

    /// Sets the count for both touch and release. See 5.7 of the [Mpr121 Data Sheet](https://www.nxp.com/docs/en/data-sheet/MPR121.pdf).
    ///
    /// In the event of an error [Mpr121Error] is returned
    #[maybe_async::maybe_async]
    pub async fn set_debounce(
        &mut self,
        trigger_debounce: DebounceNumber,
        release_debounce: DebounceNumber,
    ) -> Result<(), Mpr121Error> {
        let bits = (u8::from(release_debounce) << 4) | (u8::from(trigger_debounce));
        self.write_register(Register::Debounce, bits).await?;
        Ok(())
    }

    /// Reads the filtered data from touch channels. Noise gets filtered out by the
    /// chip. See 5.3 in the data sheet.
    ///
    /// Note that the resulting value is only 10bit wide.
    ///
    /// Otherwise [Mpr121Error] is returned
    #[maybe_async::maybe_async]
    pub async fn get_filtered(&mut self, channel: Channel) -> Result<u16, Mpr121Error> {
        let register = Register::get_filtered_data_msb(channel);
        let result = self.read_reg16(register).await?;
        Ok(result)
    }

    /// Reads the baseline data for the channel. Note that this has only a resolution of 8bit.
    ///
    /// Otherwise [Mpr121Error] is returned
    #[maybe_async::maybe_async]
    pub async fn get_baseline(&mut self, channel: Channel) -> Result<u8, Mpr121Error> {
        //NOTE: the original reads a 8bit value and left shifts 2bit.
        //      While the shift is correct the data sheet mentions:
        //
        //      Although internally the baseline value is 10-bit,
        //      users can only access the 8 MSB of the 10-bit baseline value through the
        //      baseline value registers. The read out from the baseline register must
        //      be left shift two bits before comparing it with the 10-bit
        //      electrode data.
        //
        //      reading only 8bit and shifting 2bit effectively reduces the resolution to
        //      6bit, since we loose the 2MSB.
        //
        //      Therefore we read 16bit, mask out the top 6, and then shift
        let register = Register::get_baseline(channel);
        let mut value = self.read_reg16(register).await?;
        value &= 0b00000011_11111100;
        let cast = (value << 2)
            .try_into() // If this fails to convert into u8
            .map_err(|_| Mpr121Error::DataConversionError(register))?;
        Ok(cast)
    }

    /// Reads the *touched* state of all channels. Returns a u16 where each bit 0..12 indicates whether the
    /// pin is touched or not. Use bit shifting / masking to generate a mask, or, if only one sensor's value is
    /// needed, use [get_touch_state](Self::get_sensor_touch).
    ///
    /// Otherwise [Mpr121Error] is returned
    #[maybe_async::maybe_async]
    pub async fn get_touched(&mut self) -> Result<u16, Mpr121Error> {
        //mask upper four bits returns the rest
        let unmasked = self.read_reg16(Register::TouchStatus0_7).await?;
        Ok(unmasked & 0x0fff)
    }

    ///Returns the touch state of the given sensor.
    ///
    /// Otherwise [Mpr121Error] is returned
    #[maybe_async::maybe_async]
    pub async fn get_sensor_touch(&mut self, channel: Channel) -> Result<bool, Mpr121Error> {
        let result = self.get_touched().await?;
        Ok(result & channel.get_bit_mask() > 0)
    }
}
