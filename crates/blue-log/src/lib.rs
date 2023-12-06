pub fn init_tracing(debug: &Option<bool>) {
    let debug = debug.unwrap_or(false);

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .without_time()
        .with_file(false)
        .with_line_number(false)
        .with_thread_ids(false)
        .with_target(false)
        .with_max_level(tracing::Level::INFO)
        .finish();

    let debug_subscriber = tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(true)
        .with_max_level(tracing::Level::TRACE)
        .finish();

    if debug {
        tracing::subscriber::set_global_default(debug_subscriber).unwrap();
    } else {
        tracing::subscriber::set_global_default(subscriber).unwrap();
    }

    tracing::debug!("Running Blue in debug mode");
}
