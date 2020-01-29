static SPACE: usize = 4;
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

    pub fn with_path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }

    pub fn build(self) -> Self {
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
        self
    }

    pub(crate) fn handle_log(&self, log: &str) {
        if self.path.is_some() {
            info!("{}", log);
        }
        if self.stdout {
            println!("{}", log);
        }
    }
}

impl crate::Backend for Logger {
    fn app_started(&self) {
        self.handle_log("ObserverLogger: App Started");
    }

    fn app_ended(&self) {
        self.handle_log("ObserverLogger: App Ended");
    }
    fn context_created(&self, id: &str) {
        self.handle_log(&format!("ObserverLogger: Context created with id: {}", id));
    }

    fn context_ended(&self, ctx: &crate::Context) {
        let log = if self.stdout || self.path.is_some() {
            print_context(ctx)
        } else {
            "".to_string()
        };
        self.handle_log(&format!(
            "ObserverLogger: Context ended with id: {}",
            ctx.id()
        ));
        self.handle_log(&log);
    }

    fn span_created(&self, id: &str) {
        self.handle_log(&format!("ObserverLogger: Span created with id: {}", id));
    }
    fn span_data(&self, _key: &str, _value: &str) {}
    fn span_ended(&self) {
        self.handle_log(&format!("ObserverLogger: Span ended"));
    }
}

pub(crate) fn print_context(ctx: &crate::Context) -> String {
    let mut writer = "".to_string();
    let frame = ctx.span_stack.borrow();
    if let Some(frame) = frame.first() {
        let dur = frame
            .end_time
            .as_ref()
            .unwrap_or(&chrono::Utc::now())
            .signed_duration_since(frame.start_time);
        writer.push_str(&format!(
            "context: {} [{}ms, {}]\n",
            ctx.id(),
            dur.num_milliseconds(),
            frame.start_time
        ));
        print_span(&mut writer, &frame.sub_frames, SPACE);
    }
    writer
}

pub(crate) fn print_span(writer: &mut String, spans: &Vec<crate::span::Span>, space: usize) {
    for span in spans.iter() {
        let dur = span
            .end_time
            .as_ref()
            .unwrap_or(&chrono::Utc::now())
            .signed_duration_since(span.start_time);
        writer.push_str(&format!(
            "{:>space$}{}: {}ms\n",
            "",
            span.id,
            dur.num_milliseconds(),
            space = space
        ));
        for (key, value) in span.breadcrumbs.iter() {
            writer.push_str(&format!(
                "{:>space$}@{}: {}\n",
                "",
                key,
                value,
                space = space + SPACE
            ));
        }
        if let Some(success) = span.success {
            writer.push_str(&format!(
                "{:>space$}@@success: {}\n",
                "",
                success,
                space = space + SPACE
            ));
        }
        if span.logs.len() > 0 {
            writer.push_str(&format!("{:>space$}logs:\n", "", space = space + SPACE));
            for log in span.logs.iter() {
                let dur = log
                    .0
                    .signed_duration_since(span.start_time)
                    .num_milliseconds();
                writer.push_str(&format!(
                    "{:>space$} - {}ms: {log}\n",
                    "",
                    dur,
                    log = log.1,
                    space = space + SPACE + 2,
                ));
            }
        }
        print_span(writer, &span.sub_frames, space + SPACE);
    }
}
