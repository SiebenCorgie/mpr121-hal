//! # MPR121-Hal
//!
//! This crate follows the [Adafruit implementation](https://github.com/adafruit/Adafruit_MPR121) closely but exposes it in terms of the embedded-hal project.
//!
//! The main responsibility of this crate is returning the current on/off state of all the (up to) 12 pins.
//!
//! The chip's data sheet can be found [here](https://www.nxp.com/docs/en/data-sheet/MPR121.pdf). The implementation however mostly mirrors the Adafruit implementation,
//! since this is probably the most widely used one.
//!
#![deny(unsafe_code, warnings)]
#![no_std]

use num_enum::{IntoPrimitive, TryFromPrimitive};
use registers::Register;
use strum_macros::EnumIter;

mod communications;
pub mod mpr121;
mod registers;

#[cfg(all(feature = "sync", feature = "async"))]
compile_error!("You cannot use both sync and async features at the same time. Please choose one.");

#[cfg(all(not(feature = "async"), not(feature = "sync")))]
compile_error!("You must enable either the sync or async feature. Please choose one.");

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Mpr121Error {
    ///If an operation exceeds the channel count (typically 12).
    ChannelExceed,
    ///If a read operation failed, contains the address that failed.
    ReadError(Register),
    ///If a write operation failed, contains the address that failed.
    WriteError(Register),
    ///If sending the reset signal failed, contains the register that failed.
    ResetFailed { was_read: bool, reg: Register },
    ///If the reset did not happen as expected. if ovcp is set, the reset failed because over-current protection
    /// is active.
    InitFailed { over_current_protection: bool },
}

///The four values the sensor can be addressed as. Note that the address of the device is determined by
/// where the `ADDR` pin is connected to. Default is used if no connection, or a connection to `VSS` is made.
///
/// Have a look at page 4 "serial communication" for further specification.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive)]
pub enum Mpr121Address {
    Default = 0x5a,
    Vdd = 0x5b,
    Sda = 0x5c,
    Scl = 0x5d,
}

#[repr(u8)]
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive, TryFromPrimitive, EnumIter,
)]
pub enum Channel {
    Channel0,
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7,
    Channel8,
    Channel9,
    Channel10,
    Channel11,
}
pub const NUM_TOUCH_CHANNELS: u8 = 12;

impl Channel {
    pub fn get_mask(self) -> u16 {
        1 << u8::from(self)
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive, TryFromPrimitive)]
pub enum DebounceNumber {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}
/// Threshold values for the touch and release threshold
pub const DEFAULT_TOUCH_THRESHOLD: u8 = 12;
pub const DEFAULT_RELEASE_THRESOLD: u8 = 6;
