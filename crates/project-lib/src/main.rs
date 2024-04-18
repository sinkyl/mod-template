// #[cfg(feature = "slint-ui")]
slint::include_modules!();
fn main() {
    HelloWorld::new().unwrap().run().unwrap();
}
