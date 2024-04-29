fn main() {
    println!("Hello, world!");
    milans_rust_logging::configure("resources/log.yml").unwrap();
    println!("{}",milans_rust_core::math::primality::prime_sieve(1<<32).len());
}
