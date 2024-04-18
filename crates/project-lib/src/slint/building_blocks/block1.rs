// use crate::components::comp1; // not accessible from the slint macro

use serde::{Deserialize, Serialize};

// struct declared in the same file are accessible, but this most be removed in the future
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Block1{
    pub title: String
}

impl Default for Block1 {
    fn default() -> Self {
        Block1 {
            title: "Block1 default title value".to_string()
        }
    }
}


slint::slint!{
    export component HelloWorld {
        Text {
            text: "hello world within Block1";
            color: green;
        }
    }
}
