use colored::Colorize;

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

    pub fn build(self) -> Box<Self> {
        if let Some(path) = &self.path {
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
        Box::new(self)
    }

    pub fn handle_log(&self, log: &str) {
        if self.path.is_some() {
            info!("{}", log);
        }
        if self.stdout {
            println!("{}", log);
        }
    }
}

pub fn is_replay() -> bool {
    std::env::args().any(|e| e == "--replay")
}

impl crate::Backend for Logger {
    fn context_ended(&self, ctx: &crate::Context) {
        if !is_replay() {
            println!("{}", print_context(ctx));
        }

        // let log = if self.stdout || self.path.is_some() {
        //     print_context(ctx)
        // } else {
        //     "".to_string()
        // };
        // self.handle_log(&log);
        // println!("trace without data:\n{}", ctx.trace_without_data(false));
        // println!(
        //     "trace without transient data:\n{}",
        //     ctx.trace_without_data(true)
        // );
    }
}

pub(crate) fn print_context(ctx: &crate::Context) -> String {
    let mut buffer = "".to_string();
    buffer.push_str(&format!(
        "context: {} [{}] ",
        ctx.id(),
        ctx.created_on.to_rfc3339()
    ));
    for span in ctx.span_stack.borrow().iter() {
        print_span(&mut buffer, span, 0);
    }
    buffer
}

pub(crate) fn print_span(buffer: &mut String, span: &crate::Span, space: usize) {
    buffer.push_str(&format!(
        "{}: {}\n",
        {
            let span_id = if span.duration() > std::time::Duration::from_millis(1) {
                span.id.red()
            } else {
                span.id.green()
            };
            span.success
                .map(|v| {
                    if v {
                        span_id.clone()
                    } else {
                        span_id.clone().underline()
                    }
                })
                .unwrap_or_else(|| span_id.bold())
        },
        elapsed(span.duration()),
    ));

    for (ts, item) in span.items.iter() {
        let d = elapsed(ts.0);
        match item {
            crate::SpanItem::Log { message } => {
                buffer.push_str(&format!(
                    "{:_>space$}- {}: {message}\n",
                    "",
                    d,
                    message = message,
                    space = space,
                ));
            }
            crate::SpanItem::Field { name, value } => {
                buffer.push_str(&format!(
                    "{:_>space$}- {}: {}={}\n",
                    "",
                    d,
                    name,
                    if let serde_json::Value::String(s) = value {
                        s.to_string()
                    } else {
                        value.to_string()
                    },
                    space = space
                ));
            }
            crate::SpanItem::TransientField { name, value } => {
                buffer.push_str(&format!(
                    "{:_>space$}- {}: {}:={}\n",
                    "",
                    d,
                    name,
                    if let serde_json::Value::String(s) = value {
                        s.to_string()
                    } else {
                        value.to_string()
                    },
                    space = space
                ));
            }
            crate::SpanItem::Query {
                query,
                bind,
                result,
            } => {
                buffer.push_str(&format!(
                    "{:_>space$}- query: {}\n",
                    "",
                    query,
                    space = space
                ));
                if let Some(bind) = bind {
                    buffer.push_str(&format!(
                        "{:_>space$}   bind: {}\n",
                        "",
                        bind,
                        space = space
                    ));
                }
                match result {
                    Ok(rows) => buffer.push_str(&format!(
                        "{:_>space$}   rows: {}\n",
                        "",
                        rows,
                        space = space
                    )),
                    Err(e) => {
                        buffer.push_str(&format!("{:_>space$}  error: {}\n", "", e, space = space))
                    }
                };
            }
            crate::SpanItem::Frame(inner) => {
                buffer.push_str(&format!("{:_>space$}- {}: ", "", d, space = space));
                print_span(buffer, inner, space + SPACE);
            }
        }
    }

    if let Some(result) = &span.result {
        buffer.push_str(&format!(
            "{:_>space$}result: {}\n",
            "",
            result,
            space = SPACE + space - 2
        ));
    }

    if let Some(err) = &span.err {
        buffer.push_str(&format!(
            "{:_>space$}error: {}\n",
            "",
            err,
            space = SPACE + space - 2
        ));
    }
}

pub fn elapsed(d: std::time::Duration) -> String {
    let nanos = d.subsec_nanos();
    let fraction = match nanos {
        t if nanos < 1000 => format!("{: >3}ns", t),
        t if nanos < 1_000_000 => format!("{: >3.*}µs", 0, f64::from(t) / 1000.0),
        t => format!("{: >3.*}ms", 0, f64::from(t) / 1_000_000.0),
    };
    let secs = d.as_secs();
    match secs {
        _ if secs == 0 => fraction,
        t if secs < 5 => format!("{}.{:03}s", t, nanos / 1000),
        t if secs < 60 => format!("{}.{:03}s", t, nanos / 1_000_000),
        t if secs < 3600 => format!("{}m {}s", t / 60, t % 60),
        t if secs < 86400 => format!("{}h {}m", t / 3600, (t % 3600) / 60),
        t => format!("{}s", t),
    }
}
