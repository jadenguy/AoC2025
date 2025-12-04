fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_print_hello_world() {
        // This test checks if the main function runs without panicking.
        // Capturing stdout in Rust tests is non-trivial, so we just check for no panic.
        assert!(std::panic::catch_unwind(|| super::main()).is_ok());
    }
}
