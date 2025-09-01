use axum::{body::Body, extract::Request, http::Response, routing::get, Router};
use tower_http::{
    classify::ServerErrorsFailureClass, 
    services::ServeDir, 
    trace::TraceLayer};
use crate::{
    handlers::{auth::{self, post_log_in_handler, post_sign_up_handler}, public}, state::AppState};
use std::{time::Duration};
use tracing::{self, Span};


pub fn router(app_state: AppState) -> Router{

    /* #region ROUTERS */
    let app = Router::new()
        // `GET /` goes to `public::home`
        .route("/", get(public::home))
        // `GET /login` goes to `auth::login`
        .route("/login", get(auth::login).post(post_log_in_handler))
        // 'GET /about' goes to 'public::about'
        .route("/about", get(public::about))
        // `GET /contact` goes to `public::contact`
        .route("/contact", get(public::contact))
        // `GET /signup` goes to `auth::signup`
        .route("/signup", get(auth::signup).post(post_sign_up_handler))
        // `GET /server-error` for testing the 500 page
        .route("/servererror", get(public::servererror))
        // Serve static files from the `static` directory
        .nest_service("/static", ServeDir::new("static"))
        // Fallback for non-existent routes
        .fallback(public::notfound)
        // Add the state to the router
        .with_state(app_state)
        // Add a tracing layer
        .layer(TraceLayer::new_for_http()
            .make_span_with(|_: &Request<Body>| tracing::info_span!("http-request"))
            .on_request(on_request)
            .on_response(on_response)
            .on_failure(on_failiure)
        );
    /* #endregion */

    app
}

fn on_request(request: &Request<Body>,_: &Span) {
    tracing::info!("-> Request initiated: method {} path {}", 
    request.method(), 
    request.uri().path()
);
}

fn on_response(response: &Response<Body>, latency: Duration, _: &Span) {
    tracing::info!("<- Response generated: status {} in {:?}", 
    response.status(), 
    latency
)
}

fn on_failiure(error:ServerErrorsFailureClass, latency: Duration, _: &Span) {
    tracing::error!("-x- Request failed: {:?} after {:?}", error, latency)
}
