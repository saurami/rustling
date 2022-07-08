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
    let minutes_lasagna_cooked;
    minutes_lasagna_cooked = preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven;
    minutes_lasagna_cooked
}

fn main() {
    println!("{}", remaining_minutes_in_oven(30));
    println!("{}", preparation_time_in_minutes(2));
    println!("{}", elapsed_time_in_minutes(3, 20));
}
