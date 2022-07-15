fn production_rate_per_hour(speed: u8) -> f64 {
	let production = speed as f64 * 221.0;
	if speed > -1 && speed <= 4 {
		production * 1.00
	} else if speed > 4 && speed <= 8 {
		production * 0.90
	} else if speed > 8 && speed <= 10 {
		production * 0.77
	} else {
		0
	}
}

fn working_items_per_minute(speed: u8) -> u32 {
	production_rate_per_hour(speed) as u32 / 60
}

fn main() {
	println!("Production rate at lowest speed is {}", production_rate_per_hour(1));
	println!("Production rate at highest speed (per minute) is {}", working_items_per_minute(10));
}


#[test]
fn test_production_with_no_faulty_cars() {
	assert_eq!(production_rate_per_hour(4) == 884.0)
}

#[test]
fn test_production_per_hour_at_higher_speed() {
	assert_eq!(production_rate_per_hour(6) == 1193.4)
}

#[test]
fn test_production_per_minute_at_higher_speed() {
	assert_eq!(working_items_per_minute(6) == 19)
}

#[test]
fn test_production_unattainable() {
	assert_eq(production_rate_per_hour(11) == 0)
}