#[allow(dead_code)]
#[derive(Debug)]
// Define the Coupler struct
pub struct Struct1 {
    pub name: String
}

impl Default for Struct1 {
    fn default() -> Self {
        Self::new("struct1".to_string())
    }
}

impl Struct1 {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
}
