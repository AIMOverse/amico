use tracing_subscriber::fmt;
use tracing_subscriber_wasm::MakeConsoleWriter;

pub fn init() {
    console_error_panic_hook::set_once();

    fmt()
        .with_writer(
            // To avoide trace events in the browser from showing their
            // JS backtrace, which is very annoying, in my opinion
            MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG),
        )
        // For some reason, if we don't do this in the browser, we get
        // a runtime error.
        .without_time()
        .init();
}
