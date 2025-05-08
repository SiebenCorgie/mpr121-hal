# MPR121-hal

<!-- markdown-link-check-disable -->
[![crates.io](https://img.shields.io/crates/v/mpr121-hal.svg)](https://crates.io/crates/mpr121-hal)
<!-- markdown-link-check-enable -->
[![docs.rs](https://img.shields.io/docsrs/mpr121-hal?style=plastic)](https://docs.rs/mpr121-hal/latest/)
[![dependency status](https://deps.rs/repo/gitlab/tendsinmende/mpr121-hal/status.svg)](https://deps.rs/repo/gitlab/tendsinmende/mpr121-hal)

[Mpr121](https://www.nxp.com/docs/en/data-sheet/MPR121.pdf) proximity capacitive touch sensor controller driver.
Used for instance on the [Adafruit Mpr121 module](https://www.adafruit.com/product/1982).

## Overview
<!-- markdown-link-check-disable -->
The crate uses the [embeded-hal](https://crates.io/crates/embedded-hal) crate to provided a generic implementation for multiple HALs alongside [embedded-hal-async](https://crates.io/crates/embedded-hal-async) for async support.
It uses [maybe-async](https://crates.io/crates/maybe-async) to provide a single API for both sync and async implementations.
Simply use either the `sync` or `async` feature to enable the desired implementation.
If both features are enabled cargo will fail to compile the project.
<!-- markdown-link-check-enable -->

The implementation provides a similar API to the C++ Adafruit library.
It should work with similar boards using the Mpr121 as well.

## Examples

Within the [examples](./examples/) directory there is an [ESP32S3](https://esp32s3.com/tinys3.html) example using the [MPR121 Adafruit Breakout Board](https://www.adafruit.com/product/1982) and the [Embassy Framework](https://github.com/embassy-rs/embassy) that runs in complete Async.

There is a [Simple Sync Script](./examples/os_sync_basic.rs) that runs on Mac/Linux using a [FTD232H I2C Adapter](https://shop.pimoroni.com/products/adafruit-ft232h-breakout-general-purpose-usb-to-gpio-spi-i2c?variant=1004798821) that uses the library in Synchronous mode.
You may need to Install the following sys dependencies to run the sync example.
You do not need any sys dependencies for the esp32 example or to use the driver on any other platform:

### MacOS

```bash
brew install libusb
brew install libftdi
```

### Linux

```bash
sudo apt update
sudo apt install libftdi1 libftdi1-dev
```

Then to run the example do the following:

```bash
cargo run --example os_sync_basic --no-default-features --features sync
```

More information on this can be found [here](https://github.com/dysonltd/tmag5273/blob/main/examples/README.md).

## Running your Linter Locally

This project uses [MegaLinter](https://github.com/oxsecurity/megalinter) which provides linters for various different file formats and languages. When a Pull request to main is done, the linters will run and ensure the codebase is in good standing. It is recommended that you run the linter locally beforehand as it can sometimes autofix common mistakes.

```bash
npx mega-linter-runner
```

You will need to have docker and Node installed to use this, more information can be found on their [repo](https://github.com/oxsecurity/megalinter)

### Issues with rust fmt

Currently at the time of this commit `rust fmt` is not supported as part of MegaLinter, thus to ensure it is correctly formatted we have added an extra build stage which can be seen [here](./.github/workflows/mega-linter.yaml). You can run this locally using

```bash
cargo fmt --all
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
   <http://opensource.org/licenses/MIT>)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
