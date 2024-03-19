use lazy_static::lazy_static;
lazy_static! {
    static ref LOG_INITIALIZED: std::sync::Mutex<bool> = std::sync::Mutex::new(false);
}
pub mod math;
pub fn configure_log() -> Result<(), Box<dyn std::error::Error>> {
    let mut initialized = LOG_INITIALIZED.lock().unwrap();
    if *initialized {
        return Ok(());
    }
    *initialized = true;
    __configure_log()
}
fn __configure_log() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("resources/log.yaml", Default::default()).unwrap();
    Ok(())
}
#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+)  => {{
        let error=$crate::configure_log();
        log::log!(target: $target,$lvl,$($key $(:$capture)? $(= $value)?),+;$($arg)+)
        error}
    };
    (target: $target:expr, $lvl:expr, $($arg:tt)+)  => {{
        let error=$crate::configure_log();
        log::log!(target: $target,$lvl,$($arg)+);
        error}
    };
    ($lvl:expr, $($arg:tt)+)  => {{
        let error=$crate::configure_log();
        log::log!($lvl,$($arg)+);
        error}
    };
}
#[macro_export]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => {
        $crate::log!(target: $target, log::Level::Error,$($arg)+)
    };
    ($($arg:tt)+) => {
        $crate::log!(log::Level::Error,$($arg)+)
    };
}
#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => {
        $crate::log!(target: $target, log::Level::Warn,$($arg)+)
    };
    ($($arg:tt)+) => {
        $crate::log!(log::Level::Warn,$($arg)+)
    };
}
#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => {
        $crate::log!(target: $target, log::Level::Info,$($arg)+)
    };
    ($($arg:tt)+) => {
        $crate::log!(log::Level::Info,$($arg)+)
    };
}
#[cfg(feature = "debug")]
#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => {
        $crate::log!(target: $target, log::Level::Debug,$($arg)+)
    };
    ($($arg:tt)+) => {
        $crate::log!(log::Level::Debug,$($arg)+)
    };
}
#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => {
        Ok::<(), Box<dyn std::error::Error>>(())
    };
    ($($arg:tt)+) => {
        Ok::<(), Box<dyn std::error::Error>>(())
    };
}
#[cfg(feature = "debug")]
#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => {
        $crate::log!(target: $target, log::Level::Trace,$($arg)+)
    };
    ($($arg:tt)+) => {
        $crate::log!(log::Level::Trace,$($arg)+)
    };
}
#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => {
        Ok::<(), Box<dyn std::error::Error>>(())
    };
    ($($arg:tt)+) => {
        Ok::<(), Box<dyn std::error::Error>>(())
    };
}
