use project_lib::slint::widgets::Widget1;
use project_lib::prelude::*;

fn main() {
    let widget1: Widget1 = Widget1::default();
    println!("{}", widget1.block1.title);

    let mut comp1: Comp1 = Comp1::default();
    println!("Comp1 default id: {}", comp1.id);
    println!("Comp1 property comp2 object default id: {}", comp1.id);

    comp1.id = 2;
    println!("Comp1 property comp2 object updated id: {}", comp1.id);

    let comp2: Comp2 = Comp2::default();
    println!("Comp2 default id: {}", comp2.id);
}
