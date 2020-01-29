
pub struct Logger {
    path: Option<String>,
    stdout: bool,
    stderr: bool,
}

impl Logger {
    pub fn builder() -> Self {
        Logger {
            path: None,
            stdout: false,
            stderr: false,
        }
    }

    pub fn with_stdout(mut self) -> Self {
        self.stdout = true;
        self
    }

    pub fn with_stderr(mut self) -> Self {
        self.stderr = true;
        self
    }

    pub fn build(self) {
        let path = self.path.as_ref().expect("Logger file path is provided");
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
}

impl crate::Backend for Logger {
    fn app_started(&self) {}
    fn app_ended(&self) {}
    fn context_created(&self, id: &str) {}
    fn context_ended(&self, ctx: &crate::Context) {}
    fn span_created(&self, id: &str) {}
    fn span_data(&self, key: &str, value: &str) {}
    fn span_ended(&self) {}
}
