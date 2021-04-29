#[derive(Debug)]
pub struct Duration(pub std::time::Duration);

impl serde::Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.0.as_nanos() as u64)
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum SpanItem {
    Log {
        message: &'static str,
    },
    Field {
        name: &'static str,
        value: serde_json::Value,
    },
    TransientField {
        name: &'static str,
        value: serde_json::Value,
    },
    Query {
        query: String,
        bind: Option<String>,
        result: Result<usize, String>,
    },
    Frame(Span),
}

#[derive(Serialize)]
pub struct Span {
    pub id: String,
    key: String,
    pub items: Vec<(Duration, SpanItem)>,
    pub success: Option<bool>,
    pub result: Option<serde_json::Value>,
    pub err: Option<String>,
    #[serde(skip_serializing)]
    pub created_on: std::time::Instant,
    pub duration: Option<Duration>,
}

impl Clone for Span {
    fn clone(&self) -> Self {
        Span::new(&self.id)
    }
}

impl std::fmt::Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Span")
            .field("id", &self.id)
            .field("key", &self.key)
            .field("items", &self.items)
            .field("success", &self.success)
            .field("result", &self.result)
            .field("err", &self.err)
            .field("start_time", &self.created_on)
            .field("duration", &self.duration)
            .finish()
    }
}

impl Span {
    pub fn duration(&self) -> std::time::Duration {
        self.duration
            .as_ref()
            .map(|v| v.0)
            .unwrap_or_else(|| std::time::Instant::now().duration_since(self.created_on))
    }

    pub fn new(id: &str) -> Span {
        Span {
            id: id.to_owned(),
            key: uuid::Uuid::new_v4().to_string(),
            items: Vec::new(),
            success: None,
            result: None,
            err: None,
            created_on: std::time::Instant::now(),
            duration: None,
        }
    }
    pub(crate) fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn end(&mut self) -> &mut Self {
        // TODO: assert .ended_on is null
        self.duration = Some(Duration(
            std::time::Instant::now().duration_since(self.created_on),
        ));
        self
    }

    pub fn set_result(&mut self, result: impl serde::Serialize) -> &mut Self {
        // TODO: assert .result is null
        self.result = Some(json!(result));
        self
    }

    pub fn set_success(&mut self, is_success: bool) -> &mut Self {
        // TODO: assert .success is null
        self.success = Some(is_success);
        self
    }

    pub fn set_err(&mut self, err: Option<String>) -> &mut Self {
        // TODO: assert .err is null
        self.err = err;
        self
    }

    pub fn add_sub_frame(&mut self, created_on: std::time::Instant, frame: Span) {
        self.items.push((
            Duration(created_on.duration_since(self.created_on)),
            SpanItem::Frame(frame),
        ));
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn add_log(&mut self, log: &'static str) {
        self.items.push((
            Duration(std::time::Instant::now().duration_since(self.created_on)),
            SpanItem::Log { message: log },
        ))
    }

    //adding breadcrumbs
    pub fn add_breadcrumbs(&mut self, name: &'static str, value: serde_json::Value) {
        self.items.push((
            Duration(std::time::Instant::now().duration_since(self.created_on)),
            SpanItem::Field { name, value },
        ))
    }

    pub fn add_transient_field(&mut self, name: &'static str, value: serde_json::Value) {
        self.items.push((
            Duration(std::time::Instant::now().duration_since(self.created_on)),
            SpanItem::TransientField { name, value },
        ))
    }

    pub fn add_query(
        &mut self,
        query: String,
        bind: Option<String>,
        result: Result<usize, String>,
    ) {
        self.items.push((
            Duration(std::time::Instant::now().duration_since(self.created_on)),
            SpanItem::Query {
                query,
                bind,
                result,
            },
        ))
    }
}
