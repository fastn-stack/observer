use crate as observer;
use diesel::prelude::*;
use diesel::query_builder::QueryBuilder;

pub struct DebugConnection {
    pub conn: diesel::PgConnection,
}

pub type OConnection = DebugConnection;

lazy_static! {
    pub static ref PG_POOLS: antidote::RwLock<
        std::collections::HashMap<String, r2d2::Pool<r2d2_diesel::ConnectionManager<OConnection>>>,
    > = antidote::RwLock::new(std::collections::HashMap::new());
}

impl diesel::connection::SimpleConnection for OConnection {
    fn batch_execute(&self, query: &str) -> QueryResult<()> {
        self.conn.batch_execute(query)
    }
}

impl OConnection {
    fn new(url: &str) -> ConnectionResult<Self> {
        Ok(DebugConnection {
            conn: diesel::PgConnection::establish(url)?,
        })
    }
}

fn log_query(q: &str, result: Result<usize, String>) {
    if q == "SELECT 1" {
        crate::observe_span_id("select_1");
        return;
    };

    let (query, bind) = match q.find("-- binds: ") {
        Some(idx) => q.split_at(idx),
        None => (q, ""),
    };

    {
        let (operation, table) = crate::sql_parse::parse_sql(query);
        if table.is_empty() {
            crate::observe_span_id(&format!("db__{}", operation.as_str()));
        } else {
            crate::observe_span_id(&format!(
                "db__{}__{}",
                operation.as_str(),
                table.replace("\"", "")
            ));
        };
    }

    let bind = if bind.is_empty() {
        None
    } else {
        Some(bind.replacen("-- binds: ", "", 1))
    };

    crate::observe_query(query.trim().to_string(), bind, result);
}

impl diesel::connection::Connection for OConnection {
    type Backend = diesel::pg::Pg;
    type TransactionManager = diesel::connection::AnsiTransactionManager;

    #[observed(namespace = "observer__pg")]
    fn establish(url: &str) -> ConnectionResult<Self> {
        OConnection::new(url)
    }

    #[observed(namespace = "observer__pg")]
    fn execute(&self, query: &str) -> QueryResult<usize> {
        match self.conn.execute(query) {
            Ok(i) => {
                log_query(query, Ok(i));
                Ok(i)
            }
            Err(e) => {
                log_query(query, Err(e.to_string()));
                Err(e)
            }
        }
    }

    #[observed(namespace = "observer__pg")] // Will not use any namespace here because whitelisting by `query_by_index`
    fn query_by_index<T, U>(&self, source: T) -> QueryResult<Vec<U>>
    where
        T: diesel::query_builder::AsQuery,
        T::Query:
            diesel::query_builder::QueryFragment<diesel::pg::Pg> + diesel::query_builder::QueryId,
        diesel::pg::Pg: diesel::sql_types::HasSqlType<T::SqlType>,
        U: diesel::deserialize::Queryable<T::SqlType, diesel::pg::Pg>,
    {
        let query = source.as_query();

        let debug_query = diesel::debug_query(&query).to_string();
        match self.conn.query_by_index(query) {
            Ok(v) => {
                log_query(debug_query.as_str(), Ok(v.len()));
                Ok(v)
            }
            Err(e) => {
                log_query(debug_query.as_str(), Err(e.to_string()));
                Err(e)
            }
        }
    }

    #[observed(namespace = "observer__pg")]
    fn query_by_name<T, U>(&self, source: &T) -> QueryResult<Vec<U>>
    where
        T: diesel::query_builder::QueryFragment<diesel::pg::Pg> + diesel::query_builder::QueryId,
        U: diesel::deserialize::QueryableByName<diesel::pg::Pg>,
    {
        let query = {
            let mut qb = diesel::pg::PgQueryBuilder::default();
            source.to_sql(&mut qb)?;
            qb.finish()
        };
        match self.conn.query_by_name(source) {
            Ok(v) => {
                log_query(query.as_str(), Ok(v.len()));
                Ok(v)
            }
            Err(e) => {
                log_query(query.as_str(), Err(e.to_string()));
                Err(e)
            }
        }
    }

    #[observed(namespace = "observer__pg")]
    fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize>
    where
        T: diesel::query_builder::QueryFragment<diesel::pg::Pg> + diesel::query_builder::QueryId,
    {
        let query = {
            let mut qb = diesel::pg::PgQueryBuilder::default();
            source.to_sql(&mut qb)?;
            qb.finish()
        };
        match self.conn.execute_returning_count(source) {
            Ok(i) => {
                log_query(query.as_str(), Ok(i));
                Ok(i)
            }
            Err(e) => {
                log_query(query.as_str(), Err(e.to_string()));
                Err(e)
            }
        }
    }

    fn transaction_manager(&self) -> &Self::TransactionManager {
        self.conn.transaction_manager()
    }
}
