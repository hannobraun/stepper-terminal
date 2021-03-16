# Stepper Terminal

**Please consider supporting this project financially. More information below.**

## About

Allows you to control a stepper motor from your PC via a CLI app that sends commands to a firmware application running on an [LPC845-BRK] development board.

The firmware application uses the [Stepper] library to control the motor. It is intended to serve as a testbed and usage example for Stepper. This project and Stepper are part of [Flott], an open source toolkit for motion control software in Rust.


## Status

This project is usable, but doesn't yet expose all of the features of Stepper. Check out the [list of issues](https://github.com/flott-motion/stepper-terminal/issues) for some documented limitations. Help in filling the gaps is very welcome!


## Usage

### Set up the hardware

You need the following hardware:

- Full-size breadboard
- [LPC845-BRK] development board
- [STSPIN220 Stepper Motor Driver Carrier](https://www.pololu.com/product/2876)
- Matching stepper motor ([example](https://www.pololu.com/product/1208))
- Power supply (1.8V - 10V, depending on the motor)
- Jumper wires to make the connections

Then make the following connections:

| LPC845-BRK  | STSPIN220  | Motor | Power |
| ----------: | ---------: | ----: | ----: |
| 1 - PIO0_16 |       STBY |       |       |
| 2 - PIO0_17 |      MODE1 |       |       |
| 3 - PIO0_18 |      MODE2 |       |       |
| 4 - PIO0_19 | STEP/MODE3 |       |       |
| 5 - PIO0_20 |  DIR/MODE4 |       |       |
|    20 - GND |        GND |       |       |
|    50 - VDD |        VCC |       |       |
|             |         A1 |    A1 |       |
|             |         A2 |    A2 |       |
|             |         B2 |    B2 |       |
|             |         B1 |    B1 |       |
|             |        GND |       |     - |
|             |       VMOT |       |     + |



### Flash the firmware

From the repository root, execute:

``` bash
cd firmware
cargo embed
```

Assuming you have [cargo-embed](https://github.com/probe-rs/cargo-embed) installed, and an LPC845-BRK is connected via USB, this should flash and run the firmware, as well as open an RTT terminal with some log output.

### Run the CLI

From the repository root, execute:

``` bash
cd cli
cargo run -- move-to 2000
```

This should move the motor by 2000 steps, using a nice acceleration ramp. Please note that the argument specifies an absolute position, so executing the same command again will not result in any movement (the motor already is at step 2000).

You can learn more about the command-line interface using `--help`:

``` bash
cargo run -- --help
```

You can also use `--help` on specific subcommands:

``` bash
cargo run -- move-to --help
```


## Funding

If you're getting value out of this project, Stepper or other libraries from the [Flott] toolkit, please consider supporting us financially. Your sponsorship helps to keep the project healthy and moving forward.

[Hanno Braun][@hannobraun], maintainer and original creator of this application, is [accepting sponsorship](https://github.com/sponsors/hannobraun).


## License

This project is open source software, licensed under the terms of the [Zero Clause BSD License](https://opensource.org/licenses/0BSD) (0BSD, for short). This basically means you can do anything with the software, without any restrictions, but you can't hold the authors liable for problems.

See [LICENSE.md](https://github.com/flott-motion/stepper-terminal/blob/main/LICENSE.md) for full details.


[LPC845-BRK]: https://www.nxp.com/products/processors-and-microcontrollers/arm-microcontrollers/general-purpose-mcus/lpc800-cortex-m0-plus-/lpc845-breakout-board-for-lpc84x-family-mcus:LPC845-BRK
[Stepper]: https://crates.io/crates/stepper
[Flott]: https://flott-motion.org/

[@hannobraun]: https://github.com/hannobraun
