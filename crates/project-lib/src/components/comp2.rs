
// Re-export this modual submodule: comp2/*
pub mod struct2;
// Re-export this modual subcomponent; helps to have clean path: use components:Struct2; instead of use components:comp2:struct2:Struct2;
pub use struct2::Struct2;       // Struct2 can be accessible outside
// use self::struct2::Struct2;  // if not accessible outsite, use self to use it in this module (self because the submodule has the same name)


#[derive(Debug)]
pub struct Comp2 {
    pub id: usize,
    pub struct2: Struct2
}

impl Default for Comp2 {
    fn default() -> Self {
        Self::new(1)
    }
}

impl Comp2 {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            struct2: Default::default(),
        }
    }
}
