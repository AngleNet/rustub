use std::sync::Once;
use flexi_logger::{colored_default_format, colored_opt_format};

static TEST_LOGGER_INIT: Once = Once::new();

pub fn test_setup_logger() {
    TEST_LOGGER_INIT.call_once(|| {
        flexi_logger::Logger::try_with_env_or_str("debug").unwrap()
            .log_to_stdout()
            .format_for_stdout(colored_opt_format)
            .use_utc()
            .start()
            .expect("the logger should start");
    });
}
