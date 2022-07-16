enum LogLevel {
	Info,
	Warning,
	Error
}

fn emit_log(message_type: LogLevel, content: &str) -> String {
	match message_type {
		LogLevel::Info => info(content),
		LogLevel::Warning => warning(content),
		LogLevel::Error => error(content)
	}
}

fn info(message: &str) -> String {
	String::from("[INFO]: ".to_owned() + message)
}

fn warning(message: &str) -> String {
	String::from("[WARNING]: ".to_owned() + message)
}

fn error(message: &str) -> String {
	String::from("[ERROR]: ".to_owned() + message)
}

fn main() {
	emit_log(LogLevel::Info, "Timezone changed");
	emit_log(LogLevel::Error, "Stack Overflow");
}


#[test]
fn test_emit_info_log() {
	assert_eq!(emit_log(LogLevel::Info, "Timezone changed"), "[INFO]: Timezone changed")
}

#[test]
fn test_emit_warning_log() {
	assert_eq!(emit_log(LogLevel::Warning, "Timezone not set"), "[WARNING]: Timezone not set")
}

#[test]
fn test_emit_error_log() {
	assert_eq!(emit_log(LogLevel::Error, "Stack Overflow"), "[ERROR]: Stack Overflow")
}

#[test]
fn test_info() {
	assert!(info("Timezone changed") == "[INFO]: Timezone changed")
}

#[test]
fn test_warning() {
	assert!(warning("Timezone not set") == "[WARNING]: Timezone not set")
}

#[test]
fn test_error() {
	assert!(error("Stack Overflow") == "[ERROR]: Stack Overflow")
}
