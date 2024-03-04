// all of this from https://burgers.io/custom-logging-in-rust-using-tracing

// has some "span" stuff...
pub struct CustomLayer;

use tracing::Id;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Layer;

impl<S> Layer<S> for CustomLayer
where
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        println!("Got event!");
        println!("  level={:?}", event.metadata().level());
        println!("  target={:?}", event.metadata().target());
        println!("  name={:?}", event.metadata().name());
        for field in event.fields() {
            println!("  field={}", field.name());
        }
        println!("spans...");

        // What's about all of the spans that are in scope?
        let scope = ctx.event_scope(event).unwrap();
        for span in scope.from_root() {
            println!("an ancestor span");
            println!("  name={}", span.name());
            println!("  target={}", span.metadata().target());
        }
    }

    fn on_enter(&self, id: &Id, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        println!("Enter span: {id:?}");
    }

    fn on_exit(&self, id: &Id, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        println!("Exit span: {id:?}");
    }
}

struct PrintlnVisitor;

impl tracing::field::Visit for PrintlnVisitor {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        println!("  field={} value={:?}", field.name(), value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn init_tracing() {
        // Set up how `tracing-subscriber` will deal with tracing data.
        tracing_subscriber::registry().with(CustomLayer).init();
    }

    #[test]
    fn test_burgers() {
        init_tracing();
        crate::do_something();
        // fail to see the println's
        panic!("explicit panic!");
    }
}
