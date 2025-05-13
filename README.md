# MPR121-hal

<!-- markdown-link-check-disable -->
[![crates.io](https://img.shields.io/crates/v/mpr121-hal.svg)](https://crates.io/crates/mpr121-hal)
<!-- markdown-link-check-enable -->
[![docs.rs](https://img.shields.io/docsrs/mpr121-hal?style=plastic)](https://docs.rs/mpr121-hal/latest/)
 [![MegaLinter](https://github.com/SiebenCorgie/mpr121-hal/actions/workflows/mega-linter.yaml/badge.svg)](https://github.com/SiebenCorgie/mpr121-hal/actions/workflows/mega-linter.yaml) [![Continuous Build](https://github.com/SiebenCorgie/mpr121-hal/actions/workflows/continuous-build.yaml/badge.svg?branch=main)](https://github.com/SiebenCorgie/mpr121-hal/actions/workflows/continuous-build.yaml)

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

### Issues with Running Mega Linter on Forks

Sadly due to the way GitHub handles PAT Tokens we are unable to run all of MegaLinters functionality when processing Pull Requests from forks. It is recommended that you run the linter locally as stated above, otherwise you can [setup your own PAT token](https://github.com/marketplace/actions/megalinter#apply-fixes-issues) for your fork repository and make a PR there in which you will receive the full capability of MegaLinter.

#### PAT Token for Mega Linter running on Forks

There are Two PAT Tokens requred for full GitHub MegaLinter functionality. One is already for you which is the `secrets.GITHUB_TOKEN` however to get the full functionality you will be required to create a new PAT Token with the following settings:

- Commit Statuses: Read and Write
- Contents: Read and Write
- Pull Requests: Read and Write
- Work Flows: Read and Write

If this is your first time creating a Mega Linter PAT Token it may be recommended to create it for all Repositories so you can use it for multiple projects.

This will enable MegaLinter to create PRs and modify files.

More on this can be found here:

<https://megalinter.io/latest/install-github/>

<https://github.com/oxsecurity/megalinter/issues/4317>

#### PAT Token for Dependabot (Main Repo Only)

This repository also contains a workflow for auto merging Dependabot updates. ([Workflow](./.github/workflows/dependabot.yaml)) ([Dependabot Configuration](./.github/dependabot.yaml)).

As such it requires another PAT Token named `DEPENDABOT` with the following credentials

- Commit Statuses: Read and Write
- Contents: Read and Write
- Pull Requests: Read and Write
- Work Flows: Read and Write

More on this GitHub action can be found [here](https://github.com/marketplace/actions/dependabot-auto-merge).

### Issues with rust fmt

Currently at the time of this commit `rust fmt` is not supported as part of MegaLinter, thus to ensure it is correctly formatted we have added an extra build stage which can be seen [here](./.github/workflows/mega-linter.yaml). You can run this locally using

```bash
cargo fmt --all
```

## Running Integration Tests

At present there is limited integration tests for the driver. Generic tests that can be applied across numerous architectures/platforms are stored in a [sub crate](./tests-common/) within the repository. These are platform agnostic and are focused on the driver itself. Within the [tests](./tests/) directory are platform dependent tests which can run on the developer OS or MCU platforms.

When running the tests on the developer platform with the FT232H Breakout Board, you will need to run them single threaded and not in parallel. This is due to there only being one I2C hardware block and the locking around that. To do that, run the following command:

```bash
cargo test -- --test-threads=1
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
   <http://opensource.org/licenses/MIT>)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
