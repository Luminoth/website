use axum::{Router, routing::get};

use crate::handlers::pictures::*;
use crate::state::AppState;

pub fn init_routes(app: Router<AppState>) -> Router<AppState> {
    app.route(
        "/v1/pictures/vacation/{id}",
        get(get_pictures_vacation_handler),
    )
}
