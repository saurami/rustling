enum LogLevel {
	Info,
	Error
}

fn emit_log(message_type: LogLevel) -> String {
	match message_type {
		LogLevel::Info => {
			"[INFO] ".to_owned() + &info_message()
		},
		LogLevel::Error => {
			"[ERROR] ".to_owned() + &error_message()
		}
	}
}

fn info_message() -> String {
	String::from("Timezone changed")
}

fn error_message() -> String {
	String::from("Stack Overflow")
}

fn main() {
	emit_log(LogLevel::Info);
	emit_log(LogLevel::Error);
}


#[test]
fn test_info_message() {
	assert_eq!(info_message(), "Timezone changed")
}

#[test]
fn test_error() {
	assert_eq!(error_message(), "Stack Overflow")
}

#[test]
fn test_emit_log_with_info() {
	assert_eq!(emit_log(LogLevel::Info), "[INFO] Timezone changed")
}

#[test]
fn test_emit_log_with_error() {
	assert_eq!(emit_log(LogLevel::Error), "[ERROR] Stack Overflow")
}
