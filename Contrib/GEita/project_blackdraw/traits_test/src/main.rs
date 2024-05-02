pub struct Ui {}

impl Ui {
    pub fn new() -> Self {
        println!("Hello from UI");
        Self {}
    }
}

pub trait AdvancedUi {
    fn advanced_new() -> Self;
}

impl AdvancedUi for Ui {
    fn advanced_new() -> Self {
        println!("Hello from advanced UI");
        Self {}
    }
}

fn main() {
    let k = Ui::advanced_new();
    println!("Hello, world!");
}
