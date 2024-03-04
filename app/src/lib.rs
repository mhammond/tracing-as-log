// This is our "application" - gecko

use std::sync::Arc;
mod burgers;
mod tracer;

// udl namespace function
#[tracing::instrument]
pub fn do_something() {
    second_lib::second_lib_func()
}

// udl enum from `rust_log_forwarder.udl`
#[derive(Debug)]
pub enum Level {
    Debug,
    Error,
}

impl From<tracing::Level> for Level {
    fn from(level: tracing::Level) -> Self {
        if level == tracing::Level::ERROR {
            Level::Error
        } else if level == tracing::Level::DEBUG {
            Level::Debug
        } else {
            panic!();
        }
    }
}

impl From<Level> for tracing::Level {
    fn from(level: Level) -> Self {
        match level {
            Level::Error => tracing::Level::ERROR,
            Level::Debug => tracing::Level::DEBUG,
        }
    }
}

// Record from `rust_log_forwarder.udl`; no message, just fields.
#[derive(Debug)]
struct Record {
    level: Level,
    target: String,
    name: String,
    fields: serde_json::Value,
}

// uniffi foreign trait.
trait LogSink: Send + Sync {
    fn log(&self, record: Record);
}

// Registering a logger on desktop JS is:
/*
  let app_services_logger = Cc["@mozilla.org/appservices/logger;1"].getService(
    Ci.mozIAppServicesLogger
  );
  let logger_target = "app-services:webext_storage:sync";
  app_services_logger.register(logger_target, new LogAdapter(this._log));

which roughly becomes:
*/
// Assume singleton
pub struct AppServicesLogger {}

impl AppServicesLogger {
    fn register(&self, target: String, level: Level, sink: Arc<dyn LogSink>) {
        tracer::register_log_sink(target, level, sink);
    }
}

// needed if we want logs from crates using log::. The crates should just sll move to tracing?
#[cfg(feature = "tracing_log")]
fn init_tracing_log() {
    tracing_log::LogTracer::init().ok();
    tracing_log::log::set_max_level(tracing_log::log::LevelFilter::Debug);
}

#[cfg(not(feature = "tracing_log"))]
fn init_tracing_log() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app() {
        let logger = AppServicesLogger {};

        struct Sink;

        impl LogSink for Sink {
            fn log(&self, record: Record) {
                let Record { level, target, .. } = record;
                println!("LOG: {level:?}: {target}: {:?}", record.fields)
            }
        }
        let sink = Arc::new(Sink);
        logger.register("first_lib".to_string(), Level::Debug, sink.clone());
        logger.register("second_lib".to_string(), Level::Debug, sink.clone());
        // sadly tracing_log doesn't carry the correct target etc :( - always "log"
        logger.register("log".to_string(), Level::Debug, sink);

        // Error reporter:
        struct E;

        impl LogSink for E {
            fn log(&self, record: Record) {
                println!("ERROR: {record:?}");
            }
        }
        logger.register("error-reporter".to_string(), Level::Error, Arc::new(E));

        use tracing_subscriber::prelude::*;
        tracing_subscriber::registry()
            .with(tracer::SimpleLogLayer)
            .init();

        init_tracing_log();

        do_something();
        // fail to see the println's
        panic!("explicit panic!");
    }
}
