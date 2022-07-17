#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    weight: f32
}

impl User {
    fn new(name: String, age: u8, weight: f32) -> Self {
        User {name: name, age: age, weight: weight}
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_age(&self) -> u8 {
        self.age
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn set_name(&mut self, new_name: String) {
        self.name = new_name
    }

    fn set_age(&mut self, new_age: u8) {
        self.age = new_age
    }

    fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight
    }
}

fn main() {
    let user = User::new(String::from("Bob"), 20, 150.23);
    println!("{:?}", user);
}


struct TestContext {
    user: User,
}

fn setup() -> TestContext {
    println!("Test setup ...");

    TestContext {
        user: User::new(String::from("Bob"), 20, 150.23),
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        println!("Test teardown ...");
    }
}

#[test]
fn test_user_get_name() {
    let ctx = setup();
    assert_eq!(ctx.user.get_name(), "Bob")
}

#[test]
fn test_user_get_age() {
    let ctx = setup();
    assert_eq!(ctx.user.get_age(), 20)
}

#[test]
fn test_user_get_weight() {
    let ctx = setup();
    assert_eq!(ctx.user.get_weight(), 150.23)
}

#[test]
fn test_user_set_name() {
    let mut ctx = setup();
    ctx.user.set_name("Jane".to_string());    // no performance cost compared to String::from
    assert_eq!(ctx.user.get_name(), "Jane")
}


#[test]
fn test_user_set_age() {
    let mut ctx = setup();
    ctx.user.set_age(25);
    assert_eq!(ctx.user.get_age(), 25)
}

#[test]
fn test_user_set_weight() {
    let mut ctx = setup();
    ctx.user.set_weight(160.8);
    assert_eq!(ctx.user.get_weight(), 160.8)
}
