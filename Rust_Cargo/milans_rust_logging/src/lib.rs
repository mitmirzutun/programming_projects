
pub fn configure(configuration_file:&str) -> Result<(), Box<dyn std::error::Error>> {
    use once_cell::sync::OnceCell;
    use std::sync::Mutex;
    static IS_CONFIGURED:OnceCell<Mutex<bool>>=OnceCell::new();
    let mut is_configured=IS_CONFIGURED.get_or_init(||Mutex::new(false)).lock()?;
    if *is_configured {
        return Ok(())
    }
    *is_configured=true;
    log4rs::init_file(configuration_file,Default::default())?;
    Ok(())
}
#[cfg(not(feature="log"))]
#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => {
        Ok::<(),Box<dyn std::error::Error>>(())
    };
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => {
        Ok::<(),Box<dyn std::error::Error>>(())
    };
    ($lvl:expr,$($arg:tt)+) => {
        Ok::<(),Box<dyn std::error::Error>>(())
    };
}
#[cfg(all(feature="log",not(feature="auto-config"),not(feature="internal-config")))]
#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => {
        log::log!(target:$target,$lvl,$($key $(:$capture)? $(=$value)?),+;$($arg)+);
        Ok::<(),Box<dyn std::error::Error>>(())
    };
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => {
        log::log!(target:$target,$lvl,$($arg)+);
        Ok::<(),Box<dyn std::error::Error>>(())
    };
    ($lvl:expr,$($arg:tt)+) => {
        log::log!($lvl,$($arg)+);
        Ok::<(),Box<dyn std::error::Error>>(())
    };
}
#[cfg(all(feature="log",feature="auto-config",not(feature="internal-config")))]
#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => {
        let result=$crate::configure("resources/log.yml");
        log::log!(target:$target,$lvl,$($key $(:$capture)? $(=$value)?),+;$($arg)+);
        result
    };
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => {
        let result=$crate::configure("resources/log.yml");
        log::log!(target:$target,$lvl,$($arg)+);
        result
    };
    ($lvl:expr,$($arg:tt)+) => {
        let result=$crate::configure("resources/log.yml");
        log::log!($lvl,$($arg)+);
        result
    };
}
#[cfg(all(feature="log",not(feature="auto-config"),feature="internal-config",not(feature="trace-log")))]
#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => {
        log::log!(target:$target,$lvl,$($key $(:$capture)? $(=$value)?),+;$($arg)+);
        Ok::<(),Box<dyn std::error::Error>>(())
    };
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => {
        log::log!(target:$target,$lvl,$($arg)+);
        Ok::<(),Box<dyn std::error::Error>>(())
    };
    ($lvl:expr,$($arg:tt)+) => {
        log::log!(target:"milans-rust-library::info-logger",$lvl,$($arg)+);
        log::log!(target:"milans-rust-library::concurrent-logger",$lvl,$($arg)+);
        Ok::<(),Box<dyn std::error::Error>>(())
    };
}
#[cfg(all(feature="log",feature="auto-config",feature="internal-config",not(feature="trace-log")))]
#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => {
        let result=$crate::configure("resources/log.yml");
        log::log!(target:$target,$lvl,$($key $(:$capture)? $(=$value)?),+;$($arg)+);
        result
    };
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => {
        let result=$crate::configure("resources/log.yml");
        log::log!(target:$target,$lvl,$($arg)+);
        result
    };
    ($lvl:expr,$($arg:tt)+) => {
        let result=$crate::configure("resources/log.yml");
        log::log!(target:"milans-rust-library::info-logger",$lvl,$($arg)+);
        log::log!(target:"milans-rust-library::concurrent-logger",$lvl,$($arg)+);
        result
    };
}
#[cfg(all(feature="log",not(feature="auto-config"),feature="internal-config",feature="trace-log"))]
#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => {
        log::log!(target:$target,$lvl,$($key $(:$capture)? $(=$value)?),+;$($arg)+);
        Ok::<(),Box<dyn std::error::Error>>(())
    };
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => {
        log::log!(target:$target,$lvl,$($arg)+);
        Ok::<(),Box<dyn std::error::Error>>(())
    };
    ($lvl:expr,$($arg:tt)+) => {
        log::log!(target:"milans-rust-library::info-logger",$lvl,$($arg)+);
        log::log!(target:"milans-rust-library::concurrent-logger",$lvl,$($arg)+);
        log::log!(target:"milans-rust-library::trace-logger",$lvl,$($arg)+);
        Ok::<(),Box<dyn std::error::Error>>(())
    };
}
#[cfg(all(feature="log",feature="auto-config",feature="internal-config",feature="trace-log"))]
#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => {
        let result=$crate::configure("resources/log.yml");
        log::log!(target:$target,$lvl,$($key $(:$capture)? $(=$value)?),+;$($arg)+);
        result
    };
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => {
        let result=$crate::configure("resources/log.yml");
        log::log!(target:$target,$lvl,$($arg)+);
        result
    };
    ($lvl:expr,$($arg:tt)+) => {
        let result=$crate::configure("resources/log.yml");
        log::log!(target:"milans-rust-library::info-logger",$lvl,$($arg)+);
        log::log!(target:"milans-rust-library::concurrent-logger",$lvl,$($arg)+);
        log::log!(target:"milans-rust-library::trace-logger",$lvl,$($arg)+);
        result
    };
}
#[macro_export]
macro_rules! error {
    (target: $target:expr,$($arg:tt)+) => {
        $crate::log!(target:$target,log::Level::Error,$($arg)+)
    };
    ($($arg:tt)+) => {
        $crate::log!(log::Level::Error,$($arg)+)
    };
}
#[macro_export]
macro_rules! warn {
    (target: $target:expr,$($arg:tt)+) => {
        $crate::log!(target:$target,log::Level::Warn,$($arg)+)
    };
    ($($arg:tt)+) => {
        $crate::log!(log::Level::Warn,$($arg)+)
    };
}
#[macro_export]
macro_rules! info {
    (target: $target:expr,$($arg:tt)+) => {
        $crate::log!(target:$target,log::Level::Info,$($arg)+)
    };
    ($($arg:tt)+) => {
        $crate::log!(log::Level::Info,$($arg)+)
    };
}
#[cfg(feature="debug")]
#[macro_export]
macro_rules! debug {
    (target: $target:expr,$($arg:tt)+) => {
        $crate::log!(target:$target,log::Level::Debug,$($arg)+)
    };
    ($($arg:tt)+) => {
        $crate::log!(log::Level::Debug,$($arg)+)
    };
}
#[cfg(not(feature="debug"))]
#[macro_export]
macro_rules! debug {
    (target: $target:expr,$($arg:tt)+) => {
        Ok::<(),Box<dyn std::error::Error>>(())
    };
    ($($arg:tt)+) => {
        Ok::<(),Box<dyn std::error::Error>>(())
    };
}
#[cfg(feature="debug")]
#[macro_export]
macro_rules! trace {
    (target: $target:expr,$($arg:tt)+) => {
        $crate::log!(target:$target,log::Level::Debug,$($arg)+)
    };
    ($($arg:tt)+) => {
        $crate::log!(log::Level::Debug,$($arg)+)
    };
}
#[cfg(not(feature="debug"))]
#[macro_export]
macro_rules! trace {
    (target: $target:expr,$($arg:tt)+) => {
        Ok::<(),Box<dyn std::error::Error>>(())
    };
    ($($arg:tt)+) => {
        Ok::<(),Box<dyn std::error::Error>>(())
    };
}