use std::fmt::Display;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq)]
pub struct TestLog {
    pub level: log::Level,
    pub message: String,
}
impl Display for TestLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "level: {}, message: {}", self.level, self.message)
    }
}

#[derive(Clone, Debug)]
pub struct TestLogger {
    pub logs: Arc<Mutex<Vec<TestLog>>>,
}
impl Display for TestLogger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let logs = self.logs.lock().unwrap();
        let log_lines = logs.iter().map(|log| log.to_string());
        let log_chars: usize = log_lines
            .clone()
            .map(|log| log.len())
            .fold(0, |acc, l| acc + l)
            + (log_lines.len() * 3);
        let mut logs_str = String::with_capacity(log_chars);
        for line in log_lines {
            logs_str.push_str(&line);
            logs_str.push_str("  \n");
        }
        write!(f, "logs: [\n{}]", logs_str)
    }
}

impl TestLogger {
    pub fn new() -> TestLogger {
        TestLogger {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl log::Log for TestLogger {
    fn log(&self, record: &log::Record) {
        let mut logs = self.logs.lock().unwrap();
        logs.push(TestLog {
            level: record.level(),
            message: record.args().to_string(),
        });
    }
    fn flush(&self) {}
    fn enabled(&self, _: &log::Metadata) -> bool {
        return false;
    }
}
