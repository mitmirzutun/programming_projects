pub fn test() {
    milans_rust_core::error!("Hello World");
    milans_rust_core::error!(target: "Hello World","Hello World");
    milans_rust_core::debug!("Hello World");
    milans_rust_core::debug!(target: "Hello World","Hello World");
    milans_rust_core::warn!("Hello World");
    milans_rust_core::warn!(target: "Hello World","Hello World");
    milans_rust_core::info!("Hello World");
    milans_rust_core::info!(target: "Hello World","Hello World");
    milans_rust_core::trace!("Hello World");
    milans_rust_core::trace!(target: "Hello World","Hello World");
}
