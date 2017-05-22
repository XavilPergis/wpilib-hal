#[cfg(debug_assertions)]
macro_rules! debug {
    () => { println!() };
    ($fmt:expr) => { println!($fmt) };
    ($fmt:expr, $($arg:tt)*) => { println!($fmt $($arg)*) };
}

#[cfg(not(debug_assertions))]
macro_rules! debug {
    () => ();
    ($fmt:expr) => ();
    ($fmt:expr, $($arg:tt)*) => ();
}
