pub mod mob;
pub mod item;
pub mod mob_drop_rate;

use axum::Router;
use std::sync::Arc;

pub fn router() -> Router<Arc<crate::app::AppState>> {
    Router::new()
        .merge(mob::router())
        .merge(item::router())
        .merge(mob_drop_rate::router())
}
