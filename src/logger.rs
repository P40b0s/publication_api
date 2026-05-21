use tracing::{Level, subscriber::set_global_default};

pub fn init() -> bool 
{
    let subscriber = tracing_subscriber::fmt()
        // .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| 
        // {
        //     "api-service=debug".into()
        // }))
        .with_max_level(Level::DEBUG)
        .with_line_number(true)
        .with_file(true)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE);
    match set_global_default(subscriber.finish()) {
        Ok(_) => {
            // Initialization was successful

            true
        }
        Err(_) => {
            // A global default subscriber was already set
            false
        }
    }
}