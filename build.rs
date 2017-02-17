#![feature(slice_concat_ext)]

extern crate bindgen;

use bindgen::builder;
use std::slice::SliceConcatExt;

const CONST_NAMES: [&'static str; 23] = [
    "SAMPLE_RATE_TOO_HIGH",
    "VOLTAGE_OUT_OF_RANGE",
    "LOOP_TIMING_ERROR",
    "SPI_WRITE_NO_MOSI",
    "SPI_READ_NO_MISO",
    "SPI_READ_NO_DATA",
    "INCOMPATIBLE_STATE",
    "NO_AVAILABLE_RESOURCES",
    "NULL_PARAMETER",
    "ANALOG_TRIGGER_LIMIT_ORDER_ERROR",
    "ANALOG_TRIGGER_PULSE_OUTPUT_ERROR",
    "PARAMETER_OUT_OF_RANGE",
    "RESOURCE_IS_ALLOCATED",
    "RESOURCE_OUT_OF_RANGE",
    "HAL_INVALID_ACCUMULATOR_CHANNEL",
    "HAL_COUNTER_NOT_SUPPORTED",
    "HAL_PWM_SCALE_ERROR",
    "HAL_HANDLE_ERROR",
    "HAL_SERIAL_PORT_NOT_FOUND",
    "HAL_SERIAL_PORT_OPEN_ERROR",
    "HAL_SERIAL_PORT_ERROR",
    "HAL_THREAD_PRIORITY_ERROR",
    "HAL_THREAD_PRIORITY_RANGE_ERROR"
];

fn main() {
    // Configure and generate bindings.
    let bindings = builder()
        // Use the aggregate wrapper
        .header("headers/HAL/halwrapper.h")
        // whitelist HAL and can functions
        .whitelisted_function("HAL_.*|can.*")
        // Whitelist all the constants we need
        .whitelisted_var(CONST_NAMES.join("|"))
        .generate()
        .unwrap();

    // Write the generated bindings to an output file.
    bindings.write_to_file("src/raw.rs").unwrap();
}
