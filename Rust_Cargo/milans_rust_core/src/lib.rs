pub mod math;
pub fn __configure_log() {}
#[cfg(all(feature = "log", feature = "concurrent-log", feature = "debug"))]
#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => {
        $crate::__configure_log();
        log::log!(target: $target, $lvl, $($key $(:$capture)? $(= $value)?),+;$($arg)+);
    };
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => {
        $crate::__configure_log();
        log::log!(target: $target, $lvl, $($arg)+);
    };
    ($lvl:expr, $($arg:tt)+) => {
        $crate::__configure_log();
        $crate::log!(target: "concurrent-logger",$lvl, $($arg)+);
        $crate::log!(target: "info-logger",$lvl, $($arg)+);
        $crate::log!(target: "trace-logger",$lvl, $($arg)+);
    };
}
#[cfg(all(feature = "log", feature = "concurrent-log", not(feature = "debug")))]
#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => {
        $crate::__configure_log();
        log::log!(target: $target, $lvl, $($key $(:$capture)? $(= $value)?),+;$($arg)+);
    };
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => {
        $crate::__configure_log();
        log::log!(target: $target, $lvl, $($arg)+);
    };
    ($lvl:expr, $($arg:tt)+) => {
        $crate::__configure_log();
        $crate::log!(target: "concurrent-logger",$lvl, $($arg)+);
        $crate::log!(target: "info-logger",$lvl, $($arg)+);
    };
}
#[cfg(all(feature = "log", not(feature = "concurrent-log"), feature = "debug"))]
#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => {
        $crate::__configure_log();
        log::log!(target: $target, $lvl, $($key $(:$capture)? $(= $value)?),+;$($arg)+);
    };
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => {
        $crate::__configure_log();
        log::log!(target: $target, $lvl, $($arg)+);
    };
    ($lvl:expr, $($arg:tt)+) => {
        $crate::__configure_log();
        $crate::log!(target: "info-logger",$lvl, $($arg)+);
        $crate::log!(target: "trace-logger",$lvl, $($arg)+);
    };
}
#[cfg(all(
    feature = "log",
    not(feature = "concurrent-log"),
    not(feature = "debug")
))]
#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => {
        $crate::__configure_log();
        log::log!(target: $target, $lvl, $($key $(:$capture)? $(= $value)?),+;$($arg)+);
    };
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => {
        $crate::__configure_log();
        log::log!(target: $target, $lvl, $($arg)+);
    };
    ($lvl:expr, $($arg:tt)+) => {
        $crate::__configure_log();
        $crate::log!(target: "info-logger",$lvl, $($arg)+);
    };
}
#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => {
        $crate::__configure_log();
        log::log!(target: $target, $lvl, $($key $(:$capture)? $(= $value)?),+;$($arg)+);
    };
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => {
        $crate::__configure_log();
        log::log!(target: $target, $lvl, $($arg)+);
    };
    ($lvl:expr, $($arg:tt)+) => {
        $crate::__configure_log();
        $crate::log!(target: "info-logger",$lvl, $($arg)+);
    };
}
#[macro_export]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => {
        $crate::log!(target: $target, log::Level::Error, $($arg)+);
    };
    ($($arg:tt)+) => {
        $crate::log!( log::Level::Error, $($arg)+);
    };
}
#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => {
        $crate::log!(target: $target, log::Level::Warn, $($arg)+);
    };
    ($($arg:tt)+) => {
        $crate::log!( log::Level::Warn, $($arg)+);
    };
}
#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => {
        $crate::log!(target: $target, log::Level::Info, $($arg)+);
    };
    ($($arg:tt)+) => {
        $crate::log!( log::Level::Info, $($arg)+);
    };
}
#[cfg(feature = "debug")]
#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => {
        $crate::log!(target: $target, log::Level::Debug, $($arg)+);
    };
    ($($arg:tt)+) => {
        $crate::log!( log::Level::Debug, $($arg)+);
    };
}
#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => {};
    ($($arg:tt)+) => {};
}
#[cfg(feature = "debug")]
#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => {
        $crate::log!(target: $target, log::Level::Trace, $($arg)+);
    };
    ($($arg:tt)+) => {
        $crate::log!( log::Level::Trace, $($arg)+);
    };
}
#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => {};
    ($($arg:tt)+) => {};
}
