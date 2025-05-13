#![no_std]
use embedded_hal::i2c::I2c;
use mpr121_hal::mpr121::Mpr121;

pub fn generic_test_new<I2C>(i2c: I2C)
where
    I2C: I2c,
{
    let mpr121_sensor = Mpr121::new(i2c, mpr121_hal::Mpr121Address::Default, true, true);
    assert!(mpr121_sensor.is_ok());
}

pub fn generic_test_new_default<I2C>(i2c: I2C)
where
    I2C: I2c,
{
    let mpr121_sensor = Mpr121::new_default(i2c);
    assert!(mpr121_sensor.is_ok());
}

pub fn generic_test_is_over_current_set<I2C>(i2c: I2C)
where
    I2C: I2c,
{
    let mut mpr121_sensor = Mpr121::new(i2c, mpr121_hal::Mpr121Address::Default, true, true)
        .expect("Sensor Initialisation should not fail");
    let over_current_flag = mpr121_sensor
        .is_over_current_set()
        .expect("Communication with sensor should not fail");

    assert!(!over_current_flag);
}

pub fn generic_test_get_touched<I2C>(i2c: I2C)
where
    I2C: I2c,
{
    let mut mpr121_sensor = Mpr121::new(i2c, mpr121_hal::Mpr121Address::Default, true, true)
        .expect("Sensor Initialisation should not fail");
    assert!(
        mpr121_sensor
            .get_touched()
            .expect("Communication should not fail")
            == 0
    ); // Nothing should be triggered if not connected to anything
}
