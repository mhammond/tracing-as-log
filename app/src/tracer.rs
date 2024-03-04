use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use tracing_subscriber::Layer;

use crate::LogSink;
use tracing::field::{Field, Visit};

struct LogEntry {
    level: tracing::Level,
    sink: Arc<dyn LogSink>,
}

static LOGGERS_BY_TARGET: Lazy<RwLock<HashMap<String, LogEntry>>> = Lazy::new(|| {
    let h: HashMap<String, LogEntry> = HashMap::new();

    RwLock::new(h)
});

pub fn register_log_sink(target: String, level: crate::Level, sink: Arc<dyn LogSink>) {
    LOGGERS_BY_TARGET.write().insert(
        target,
        LogEntry {
            level: level.into(),
            sink,
        },
    );
}

pub(crate) struct SimpleLogLayer;

impl<S> Layer<S> for SimpleLogLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let target = event.metadata().target();
        if let Some(entry) = LOGGERS_BY_TARGET.read().get(target) {
            let level = *event.metadata().level();
            if level <= entry.level {
                let mut fields = BTreeMap::new();
                let mut visitor = JsonVisitor(&mut fields);
                event.record(&mut visitor);
                let record = crate::Record {
                    level: level.into(),
                    target: target.to_string(),
                    name: event.metadata().name().to_string(),
                    fields: serde_json::to_value(&fields).unwrap_or_default(),
                };
                entry.sink.log(record);
            }
        }
    }
}

// from https://burgers.io/custom-logging-in-rust-using-tracing
struct JsonVisitor<'a>(&'a mut BTreeMap<String, serde_json::Value>);

impl<'a> Visit for JsonVisitor<'a> {
    fn record_f64(&mut self, field: &Field, value: f64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
        self.0.insert(
            field.name().to_string(),
            serde_json::json!(value.to_string()),
        );
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.0.insert(
            field.name().to_string(),
            serde_json::json!(format!("{:?}", value)),
        );
    }
}
