use tracing::{debug, error, event, Level};

#[tracing::instrument]
pub fn first_lib_func() {
    other_func(66)
}

fn other_func(a: u32) {
    debug!("debug other func {a}");
    error!("error other func {a}");
    event!(target: "error-reporter", Level::ERROR, "reportable oops {a}");
    log_lib::inner(a);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    #[test]
    fn it_works() {
        // This is equivalent to `env_logger::try_init().ok();`
        // Note only the `debug!()` output is seen, the `#[tracing::instrument]` is not.
        // (however, if that mattered, I expect using a different filter could work)
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::from_default_env())
            .init();

        first_lib_func();
    }
}
