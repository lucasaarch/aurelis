pub mod account;
pub mod item;

use axum::Router;
use std::sync::Arc;

pub fn router() -> Router<Arc<crate::app::AppState>> {
    Router::new().merge(account::router()).merge(item::router())
}
