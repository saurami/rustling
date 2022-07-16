enum LogLevel {
	Info,
	Error
}

fn emit_log(message_type: LogLevel) {
	match message_type {
		LogLevel::Info => {
			println!("{}", info_message());
		},
		LogLevel::Error => {
			println!("{}", error_message());
		}
	}
}

fn info_message() -> String {
	String::from("[INFO] Timezone changed")
}

fn error_message() -> String {
	String::from("[ERROR] Stack Overflow")
}

fn main() {
	emit_log(LogLevel::Info);
	emit_log(LogLevel::Error);
}


// #[test]
// fn test_info_message() {
// 	let message = LogLevel::Info;
// }

// #[test]
// fn test_warning() {

// }

// #[test]
// fn test_error() {

// }
