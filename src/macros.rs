#[macro_export]
macro_rules! increment_counter {
    ($state:ident.$property:ident, $metric:ident) => {{
        use {opentelemetry::Context, tracing::debug};

        if let Some(metrics) = &$state.$property {
            metrics.$metric.add(&Context::current(), 1, &[]);
            debug!("incremented `{}` counter", stringify!($metric));
        }
    }};
}

#[macro_export]
macro_rules! decrement_counter {
    ($state:ident.$property:ident, $metric:ident) => {{
        use {opentelemetry::Context, tracing::debug};

        if let Some(metrics) = &$state.$property {
            metrics.$metric.add(&Context::current(), -1, &[]);
            debug!("decremented `{}` counter", stringify!($metric));
        }
    }};
}
