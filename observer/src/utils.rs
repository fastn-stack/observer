pub fn logging(path: &str) {
    // let stdout = log4rs::append::console::ConsoleAppender::builder().build();
    let requests = log4rs::append::file::FileAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
            "{d} - {m}{n}",
        )))
        .append(true)
        .build(path)
        .expect("Failed to create file appender");

    let config = log4rs::config::Config::builder()
        .appender(log4rs::config::Appender::builder().build("requests", Box::new(requests)))
        .build(
            log4rs::config::Root::builder()
                .appender("requests")
                .build(log::LevelFilter::Info),
        )
        .unwrap();
    log4rs::init_config(config).expect("Failed to create logging builder");
}
