use env_logger::Builder;
use log::LevelFilter;


pub fn enable_trace() {
    Builder::new()
        .format_timestamp(None)
        .filter_level(LevelFilter::Trace)
        .try_init();
}