use diesel::{PgConnection, r2d2::ConnectionManager};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use r2d2::Pool;
use tracing::info;

pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Database(pub DbPool);

impl Database {
    pub fn new(db_uri: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(db_uri);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create database connection pool");

        match pool.get() {
            Ok(mut conn) => {
                if let Err(err) = conn.run_pending_migrations(MIGRATIONS) {
                    panic!("Failed to run database migrations: {err}");
                } else {
                    info!("Database migrations ran successfully");
                }
            }
            Err(e) => panic!("Failed to get database connection from pool: {e}"),
        }

        Self(pool)
    }

    pub fn get(&self) -> r2d2::PooledConnection<ConnectionManager<PgConnection>> {
        self.0.get().expect("Failed to get database connection")
    }

    pub fn health_check(&self) -> bool {
        self.0.get().is_ok()
    }
}