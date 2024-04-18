use serde::{Deserialize, Serialize};

use crate::slint::building_blocks::Block1;

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Widget1{
    pub block1: Block1
}

slint::slint!{
    export component HelloWorld {
        // property <Widget1> name; // not working
        Text {
            text: "hello world within Widget1";
            color: green;
        }
    }
}
