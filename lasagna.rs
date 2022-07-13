fn expected_minutes_in_oven() -> i8 {
    40
}

fn remaining_minutes_in_oven(actual_minutes_in_oven: i8) -> i8 {
    expected_minutes_in_oven() - actual_minutes_in_oven
}

fn preparation_time_in_minutes(number_of_layers: i8) -> i8 {
    number_of_layers * 2
}

fn elapsed_time_in_minutes(number_of_layers: i8, actual_minutes_in_oven: i8) -> i8 {
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}

fn main() {
    println!(expected_minutes_in_oven());
    println!(remaining_minutes_in_oven(30));
    println!(preparation_time_in_minutes(2));
    println!(elapsed_time_in_minutes(3, 20));
}


#[test]
fn test_expected_minutes_in_oven() {
    assert!(expected_minutes_in_oven() == 40)
}

#[test]
fn test_remaining_time_in_oven() {
    assert!(remaining_minutes_in_oven(30) == 10)
}

#[test]
fn test_preparation_time_in_minutes() {
    assert!(preparation_time_in_minutes(2) == 4)
}

#[test]
fn test_elapsed_time_in_minutes() {
    assert!(elapsed_time_in_minutes(3, 20) == 26)
}
