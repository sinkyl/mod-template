// #[cfg(feature = "slint")]
fn main() {
    slint_build::compile("src/slint/test.slint").unwrap();
}
