wpilib-hal
==========

This crate aims to create bindings to the hardware abstractions used in wpilib. Another crate will provide things like commands and PID loops and whatnot. A NetworkTables wrapper will be provided in another crate.

Also See:
---------

Also be sure to check out another wpilib Rust port [here](https://github.com/robotrs/rust-wpilib).
It has bindings to a bunch of WPILIB as well as some of the abstractions not provided here.

WPILibC and WPILibJ, and the library headers used in this crate can be found [here](https://github.com/wpilibsuite/allwpilib).

Progress:
---------

Designs are not final, and a ceanup pass needs to be done after everything is wrapped. During this pass, the HAL implementation will be reviewed and as many functions that can currently error as possible will be replaced with non-erroring variants.

Here is a list of thigs that currently have wrappers:
  - Onboard Accelerometer
  - Analog I/O
  - Analog Gyro
  - Analog Trigger
  - Compressor
  - Encoder
  - Interrupt Handler
  - Hardware Timer
  - Power Distribution Panel
  - Relays
  - Solenoids
  - Miscellaneous

Here is a list of things to be worked on/wrapped:
  - CAN Transmission
  - Digital Counter
  - Digital I/O
  - I2C
  - Driver Station & Joysticks
  - PWM
  - Serial I/O
  - SPI
