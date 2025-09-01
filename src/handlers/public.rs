use crate::models::templates::{AboutTemplate, ContactTemplate, HomeTemplate, NotFound, ServerError};
use crate::state::AppState;
use crate::util::HtmlTemplate;
use axum::http::StatusCode;
use axum::extract::State;
use axum::response::IntoResponse;

/* #region PAGE HANDLERS */

// Handler for home page.
pub async fn home(State(state): State<AppState>) -> impl IntoResponse {
    let mut num = state.view_count.lock().unwrap();
    *num += 1;

    let template = HomeTemplate { view_count: *num };
    HtmlTemplate(template)
}

// Handler for about page.
pub async fn about(State(state): State<AppState>) -> impl IntoResponse {
    let mut num = state.view_count.lock().unwrap();
    *num += 1;

    let template = AboutTemplate { view_count: *num };
    HtmlTemplate(template)
}

// Handler for contact page.
pub async fn contact(State(state): State<AppState>) -> impl IntoResponse {
    let mut num = state.view_count.lock().unwrap();
    *num += 1;

    let template = ContactTemplate { view_count: *num };
    HtmlTemplate(template)
}

/// Handler for the 404 Not Found page.
pub async fn notfound() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, HtmlTemplate(NotFound {}))
}

/// Handler for the 500 Internal Server Error page.
/// This is also used for testing the error page.
pub async fn servererror() -> impl IntoResponse {
   (
       StatusCode::INTERNAL_SERVER_ERROR,
       HtmlTemplate(ServerError {}),
   )
}

/* #endregion */