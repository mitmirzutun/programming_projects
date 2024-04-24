fn main() {
    println!("Hello, world!");
    milans_rust_logging::configure("resources/log.yml").unwrap();
    milans_rust_logging::configure("resources/log.yml").unwrap();
    milans_rust_logging::error!("Hello world");
    milans_rust_logging::warn!("Hello world");
    milans_rust_logging::info!("Hello world");
    milans_rust_logging::debug!("Hello world");
    milans_rust_logging::trace!("Hello world");
}
