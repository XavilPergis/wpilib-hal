use std::env;

const LINK_LIBRARIES: &'static [&'static str] = &[
    "HALAthena",
    "wpiutil",
    "FRC_NetworkCommunication",
    "RoboRIO_FRC_ChipObject",
    "NiFpga",
    "NiFpgaLv",
    "niriosession",
    "spi",
    "i2c",
    "visa",
    "NiRioSrv",
    "niriodevenum"
];

fn main() {
    // Link to the HAL libraries
    for lib in LINK_LIBRARIES {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }

    // We store our libraries in {crate_root}/lib, so direct the linker to search there
    println!("cargo:rustc-link-search=native={}/lib", env::current_dir().unwrap().display());
}
