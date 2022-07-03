fn message() -> String {
    String::from("Hello, World!")
}

fn main() {
    print!("{}", message())
}

#[test]
fn test_message() {
    assert_eq!(message(), "Hello, World!")
}
