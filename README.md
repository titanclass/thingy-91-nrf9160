# Board Support Package for the Nordic Thingy:91-nRF9160

> This board support package was initially cloned from [Nordic nRF9160-DK](https://github.com/nrf-rs/nrf9160-dk)
> and is expected to continue to be inspired by it. Thanks to [42 Technology](https://www.42technology.com/) 
> for their past and future work.

This crate is a Board Support Package (BSP). It wraps the HAL crate
(`nrf9160-hal`) for the on-board nRF9160 of the [Nordic Thingy:91](https://www.nordicsemi.com/Software-and-tools/Prototyping-platforms/Nordic-Thingy-91) reference board, 
and provides high level wrappers for the onboard features.

## Usage

You will require Rust 1.51 or higher, and the `thumbv8m.main-none-eabihf` target
installed.

```console
$ rustc --version
rustc 1.52.1 (9bc8c42bb 2021-05-09)
$ rustup target add thumbv8m.main-none-eabihf
```

To use this BSP in your own application, add as a dependency and call the
`Board::take()` function.

## Examples

To build one of the examples run:

```console
$ git clone https://github.com/titanclass/thingy-91-nrf9160
$ cd thingy-91-nrf9160
$ cargo objcopy --target=thumbv8m.main-none-eabihf --example blinky -- -O ihex target/thumbv8m.main-none-eabihf/debug/examples/blinky.hex
```

If you don't have `cargo-objcopy` installed, run:

```console
$ rustup component add llvm-tools-preview
$ cargo install cargo-binutils
```

Or you can just run objcopy manually:

```console
$ sudo apt install binutils # provides /usr/bin/objcopy on Ubuntu
$ cargo build --target=thumbv8m.main-none-eabihf --example blinky
$ objcopy -O ihex target/thumbv8m.main-none-eabihf/debug/examples/blinky target/thumbv8m.main-none-eabihf/debug/examples/blinky.hex
```

Either way you can then flash the `blinky.hex` file using SEGGER JFlashLite, or
your favourite flashing tool.

## Debugging

To debug with the Thingy:91, you will need an [nRF9160-DK](https://www.nordicsemi.com/Software-and-Tools/Development-Kits/nRF9160-DK) and SWD cable.
We recommend flashing a blinky app to the nRF9160-DK to provide some visual confirmation of which device is being flashed/debugged as the
development provides no other visual confirmation.

The nRF9160-DK has an on-board SEGGER JLink debug probe. You need to run the
SEGGER JLink-to-GDB server software, and you can then debug the board using any
GDB interface.

```console
$ /opt/SEGGER/JLink/JLinkGDBServerExe &
$ # A GUI will pop up. Select the nRF9160 device.
$ cargo build --target=thumbv8m.main-none-eabihf --example blinky
$ gdb-multiarch ./target/thumbv8m.main-none-eabihf/debug/examples/blinky
GNU gdb (Ubuntu 9.2-0ubuntu1~20.04) 9.2
Copyright (C) 2020 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
...
Resetting target
(gdb) continue
Continuing.

Breakpoint 1, main () at examples/blinky.rs:24
24      #[entry]
(gdb) bt
#0  main () at examples/blinky.rs:24
(gdb) 
```

You can also follow [this guide on using SEGGER J-Link with Visual Studio
Code](https://wiki.segger.com/J-Link_Visual_Studio_Code). The `"device"`
parameter should be set to `"nrf9160"`.

## Secure vs Non-Secure

This BSP is designed to run in non-secure mode, as should most of your
application code. You will therefore need a 'bootloader' which starts in secure
mode, moves the required peripherals into 'non-secure' world, and then jumps to
your application.

We have succesfully used Nordic's [Secure Partition
Manager](https://github.com/nrfconnect/sdk-nrf/tree/master/samples/spm) from nRF
SDK v1.5.1. SPM v1.5.1 is configured to expect your application at address
`0x0004_0000` for the thingy91_nrf9160 board, and so that is what `memory.x` must specify as the start of Flash.
Note that other version of SPM might specify a different start address!

```console
$ west init -m https://github.com/nrfconnect/sdk-nrf --mr v1.5.1 ncs
$ cd ncs
$ west update # This takes *ages*
$ cd nrf/examples/spm
$ west build --board=thingy91_nrf9160
$ west flash
```

Your nRF9160-DK will now have SPM installed between `0x0000_0000` and
`0x0003_FFFF`. Flashing your application at `0x0004_0000` should not affect SPM,
provided you do not select *erase entire chip* or somesuch!

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
