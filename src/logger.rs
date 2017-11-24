use log::{Log, LogRecord, LogLevel, LogMetadata};
use std::io::{Write, stderr};

pub struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Debug
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let _ = writeln!(
                &mut stderr(),
                "{} - {}",
                record.level(),
                record.args()
            );
            if let Ok(mut f) = ::std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("/tmp/railcarlog")
            {
                f.write_all(
                    format!{"{} = {}\n", record.level(), record.args()}
                        .as_bytes(),
                ).unwrap();
            };
        }
    }
}
