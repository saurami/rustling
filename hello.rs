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

#[test]
#[should_panic]
fn test_incorrect_message() {
    assert!(message() == "Hello World")
}
