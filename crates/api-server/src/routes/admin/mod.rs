pub mod mob;

use axum::Router;
use std::sync::Arc;

pub fn router() -> Router<Arc<crate::app::AppState>> {
    Router::new().merge(mob::router())
}
