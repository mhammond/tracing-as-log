use tracing::debug;

#[tracing::instrument]
pub fn second_lib_func() {
    first_lib::first_lib_func();
    debug!("did first_lib_func");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    #[test]
    fn my_test() {
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::from_default_env())
            .init();

        second_lib_func();
    }
}
