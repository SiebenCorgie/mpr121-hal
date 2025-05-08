#[cfg(feature = "sync")]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

use crate::{mpr121::Mpr121, registers::*, Mpr121Error};

impl<I2C: I2c> Mpr121<I2C> {
    #[maybe_async::maybe_async]
    //Write implementation. Returns an error if a read or write operation failed. The error contains the failing register.
    pub(crate) async fn write_register(&mut self, reg: u8, value: u8) -> Result<(), Mpr121Error> {
        //MPR121 must be in Stop mode for most reg writes. This is not true for all, but
        // we are conservative here.
        let mut stop_required = true;
        let ecr_register = Registers::Ecr.into();
        let addr = self.addr.into();
        //ECR and 0x73..0x71 don't need stop. makes this a bit faster
        if reg == ecr_register || (0x73..=0x7a).contains(&reg) {
            stop_required = false;
        }
        //Check in which mode we are by reading ECR.
        let ecr_state = self.read_reg8(ecr_register).await?;

        if stop_required {
            //set to stop
            let result = self.i2c.write(addr, &[ecr_register, 0x00]).await;
            result.map_err(|_| Mpr121Error::WriteError(ecr_register))?;
        }

        //actual write
        let result = self.i2c.write(addr, &[reg, value]).await;
        result.map_err(|_| Mpr121Error::WriteError(reg))?;

        //reset to old ecr state
        if stop_required {
            let result = self.i2c.write(addr, &[ecr_register, ecr_state]).await;
            result.map_err(|_| Mpr121Error::WriteError(ecr_register))?;
        }

        Ok(())
    }

    #[maybe_async::maybe_async]
    //Reads the value, returns Err, if reading failed.
    pub(crate) async fn read_reg8(&mut self, reg: u8) -> Result<u8, Mpr121Error> {
        let mut val = [0u8];
        let result = self
            .i2c
            .write_read(self.addr.into(), &[reg], &mut val)
            .await;
        if result.is_err() {
            return Err(Mpr121Error::ReadError(reg));
        }
        Ok(val[0])
    }

    #[maybe_async::maybe_async]
    //Reads the value, returns Err, if reading failed.
    pub(crate) async fn read_reg16(&mut self, reg: u8) -> Result<u16, Mpr121Error> {
        let mut val = [0u8, 0u8];
        let result = self
            .i2c
            .write_read(self.addr.into(), &[reg], &mut val)
            .await;
        if result.is_err() {
            return Err(Mpr121Error::ReadError(reg));
        }
        Ok(u16::from_le_bytes(val))
    }
}
