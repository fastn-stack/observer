pub struct Observe;

impl Observe {
    pub fn observe_with_result<F, T, E>(
        // ctx: &Context,
        table_name: &str,
        is_critical: bool,
        run: F,
    ) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>,
        E: std::fmt::Display,
        Self: Sized,
    {
        crate::start_span(table_name);
        match run() {
            Ok(r) => {
                crate::end_span(is_critical, None);
                Ok(r)
            }
            Err(e) => {
                crate::end_span(is_critical, Some(e.to_string()));
                Err(e)
            }
        }
    }

    pub fn observe_all<F, T>(table_name: &str, is_critical: bool, run: F) -> T
    where
        F: FnOnce() -> T,
        Self: Sized,
    {
        crate::start_span(table_name);
        let result = run();
        crate::end_span(is_critical, None);
        result
    }
}
