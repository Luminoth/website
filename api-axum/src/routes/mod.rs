mod downloads;
mod news;
mod pictures;
mod static_files;
mod wow;

use axum::{routing::get, Router};
use tracing::info;

use crate::handlers;
use crate::state::AppState;

pub fn init_routes(app: Router<AppState>) -> Router<AppState> {
    info!("Initializing routes...");

    // TODO: is there a cleaner way to do this?
    // it's needed for the ELB health checks
    let app = app.route("/", get(|| async {}));

    // TODO: this is ugly
    let app = downloads::init_routes(app);
    let app = news::init_routes(app);
    let app = pictures::init_routes(app);
    let app = wow::init_routes(app);

    static_files::init_routes(app).fallback(handlers::handler_404)
}
