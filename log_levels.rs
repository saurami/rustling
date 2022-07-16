enum LogLevel {
	Info,
	Error
}

fn emit_log(message_type: LogLevel) {
	match message_type {
		LogLevel::Info => {
			println!("[INFO] Timezone changed");
		},
		LogLevel::Error => {
			println!("[ERROR] Stack Overflow");
		}
	}
}

fn main() {
	emit_log(LogLevel::Info);
	emit_log(LogLevel::Error);
}
