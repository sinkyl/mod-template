use crate::components::comp1::Struct1;


#[derive(Debug)]
pub struct Struct2 {
    pub age: i32,
    pub struct1: Struct1
}


impl Default for Struct2 {
    fn default() -> Self {
        Self::new(18)
    }
}

impl Struct2 {
    pub fn new(age: i32) -> Self {
        Self {
            age,
            struct1: Default::default()
        }
    }
}
