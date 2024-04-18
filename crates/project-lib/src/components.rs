// Re-export main components/modules
pub mod comp1;
pub mod comp2;

// Re-export components to simplify their access path
pub use comp1::Struct1;
pub use comp2::Struct2;
