use tracing_subscriber::fmt::format::FmtSpan;

/// Initializes env_logger.
/// The format is:
/// `<level>  /path/to/file:<line_number>  <time>  <log_message>`
pub fn init_logging(log_level: log::LevelFilter) {
    env_logger::Builder::new().filter_level(log_level).init();

    let subscriber = tracing_subscriber::fmt()
        .with_ansi(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting tracing default failed")
}
