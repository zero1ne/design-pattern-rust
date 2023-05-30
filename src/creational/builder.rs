
#[derive(Debug, PartialEq)]
pub struct Person {
    name: String,
    age: u8,
    sex: String,
    phone_number: String,
    address: String,
    job: String,
}

impl Person {
    pub fn builder() -> PersonBuilder {
        PersonBuilder::new()
    }
}

pub struct PersonBuilder {
    name: String,
    age: u8,
    sex: String,
    phone_number: String,
    address: String,
    job: String,
}

impl PersonBuilder {
    pub fn new() -> PersonBuilder {
        PersonBuilder {
            name: "".to_string(),
            age: 0,
            sex: "".to_string(),
            phone_number: "".to_string(),
            address: "".to_string(),
            job: "".to_string(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn age(mut self, age: u8) -> Self {
        self.age = age;
        self
    }

    pub fn sex(mut self, sex: String) -> Self {
        self.sex = sex;
        self
    }

    pub fn phone_number(mut self, phone_number: String) -> Self {
        self.phone_number = phone_number;
        self
    }

    pub fn address(mut self, address: String) -> Self {
        self.address = address;
        self
    }

    pub fn job(mut self, job: String) -> Self {
        self.job = job;
        self
    }

    pub fn build(self) -> Person {
        Person {
            name: self.name,
            age: self.age,
            sex: self.sex,
            phone_number: self.phone_number,
            address: self.address,
            job: self.job,
        }
    }
}

#[test]
fn builder_test() {
    let person_from_builder = PersonBuilder::new()
        .name("alice".to_string())
        .age(20)
        .sex("Female".to_string())
        .job("Programmer".to_string())
        .address("Seoul".to_string());
}