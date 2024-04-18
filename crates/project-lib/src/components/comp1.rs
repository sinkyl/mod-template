// Re-export this modual submodule: comp1/*
pub mod struct1;
// Re-export this modual subcomponent; helps to have clean path: use components:Struct1; instead of use components:comp1:struct1:Struct1;
pub use struct1::Struct1;

use super::comp2::Comp2;



#[allow(dead_code)]
// Define the Edge struct
#[derive(Debug)]
pub struct Comp1 {
    pub id: usize,
    pub comp2: Comp2
}

impl Default for Comp1 {
    fn default() -> Self {
        Self::new(1)
    }
}

impl Comp1 {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            comp2: Default::default(),
        }
    }
}
