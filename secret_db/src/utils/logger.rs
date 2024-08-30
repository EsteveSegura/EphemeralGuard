use chrono::Local;

pub struct Logger;

impl Logger {
    pub fn log(&self, log_type: &str, payload: &str) {
        let current_time = formatted_current_time();
        println!("[{}] {} -> {}", log_type, current_time, payload);
    }
}

pub fn get_logger() -> Logger {
    Logger
}

fn formatted_current_time() -> String {
    let now = Local::now();
    now.format("%d/%m/%Y@%H:%M:%S%.3f").to_string()
}

#[macro_export]
macro_rules! log {
    ($log_type:expr, $payload:expr) => {
        {
            let logger = crate::utils::logger::get_logger();
            logger.log($log_type, $payload);
        }
    };
}