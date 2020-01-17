use crate::observe::Observe;
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

fn _connection_pool<T: Into<String>>(
    url: T,
) -> r2d2::Pool<r2d2_diesel::ConnectionManager<OConnection>> {
    let manager = r2d2_diesel::ConnectionManager::<OConnection>::new(url);
    r2d2::Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Fail to create Diesel Connection Pool")
}

pub fn connection_with_url(
    db_url: String,
) -> r2d2::PooledConnection<r2d2_diesel::ConnectionManager<OConnection>> {
    {
        if let Some(pool) = PG_POOLS.read().get(&db_url) {
            return pool.get().unwrap();
        }
    }
    match PG_POOLS.write().entry(db_url.clone()) {
        std::collections::hash_map::Entry::Vacant(e) => {
            let conn_pool = _connection_pool(db_url);
            let conn = conn_pool.get().unwrap();
            e.insert(conn_pool);
            conn
        }
        std::collections::hash_map::Entry::Occupied(e) => e.get().get().unwrap(),
    }
}

pub fn connection() -> r2d2::PooledConnection<r2d2_diesel::ConnectionManager<OConnection>> {
    connection_with_url(std::env::var("PG_DATABASE_URL").expect("DATABASE_URL not set"))
}

impl diesel::connection::Connection for OConnection {
    type Backend = diesel::pg::Pg;
    type TransactionManager = diesel::connection::AnsiTransactionManager;

    #[observed(namespace="observer::pg")]
    fn establish(url: &str) -> ConnectionResult<Self> {
        OConnection::new(url)
    }

    #[observed(namespace="observer::pg")]
    fn execute(&self, query: &str) -> QueryResult<usize> {
        let (operation, table) = crate::sql_parse::parse_sql(&query);
        crate::observe_fields::observe_string("query", &query);
        crate::observe_span_id(&format!("db__{}__{}", operation, table.replace("\"", "")));
        let r = self.conn.execute(query);
        eprintln!("ExecuteQuery: {}", query);
        r
    }

    #[observed(namespace="observer::pg")] // Will not use any namespace here because whitelisting by `query_by_index`
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
        let (operation, table) = crate::sql_parse::parse_sql(&debug_query);
        crate::observe_fields::observe_string("query", &debug_query);
        crate::observe_span_id(&format!("db__{}__{}", operation, table.replace("\"", "")));
        let r = self.conn.query_by_index(query);
        eprintln!("QueryByIndex: {}", debug_query.as_str());
        r
    }

    #[observed(namespace="observer::pg")]
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
        let (operation, table) = crate::sql_parse::parse_sql(&query);
        crate::observe_fields::observe_string("query", &query);
        crate::observe_span_id(&format!("db__{}__{}", operation, table.replace("\"", "")));
        let r = self.conn.query_by_name(source);
        eprintln!("QueryByName: {}", query.as_str());
        r
    }

    #[observed(namespace="observer::pg")]
    fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize>
    where
        T: diesel::query_builder::QueryFragment<diesel::pg::Pg> + diesel::query_builder::QueryId,
    {
        let query = {
            let mut qb = diesel::pg::PgQueryBuilder::default();
            source.to_sql(&mut qb)?;
            qb.finish()
        };
        let (operation, table) = crate::sql_parse::parse_sql(&query);
        crate::observe_fields::observe_string("query", &query);
        crate::observe_span_id(&format!("db__{}__{}", operation, table.replace("\"", "")));
        let r = self.conn.execute_returning_count(source);
        eprintln!("ExecuteReturningCount: {}", query.as_str());
        r
    }

    fn transaction_manager(&self) -> &Self::TransactionManager {
        self.conn.transaction_manager()
    }
}
