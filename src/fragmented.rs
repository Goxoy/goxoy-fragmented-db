#[derive(Debug)]
pub struct Fragmented {
    pub db_open:bool,
}

impl Fragmented {
    pub fn new() -> Self {
        Fragmented{
            db_open:true,
        }
    }
}

#[test]
fn full_test() {
    // cargo test  --lib full_test -- --nocapture
    assert!(true)
}
