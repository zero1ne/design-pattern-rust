
pub trait Target {
    fn request(&self);
}

pub struct Adaptee;

impl Adaptee {
    pub fn specific_request(&self) {
        println!("Called SpecificRequest() on Adaptee")
    }
}

pub struct Adapter {
    adaptee: Adaptee,
}

impl Target for Adapter {
    fn request(&self) {
        self.adaptee.specific_request();
    }
}

#[test]
fn test_adapter() {
    let adaptee = Adaptee;
    adaptee.specific_request();

    let target = Adapter { adaptee };
    target.request();
}