use crate::models::{
    templates::{LoginTemplate, SignupTemplate},
    user_form_models::AuthFormModel,
};
use crate::state::AppState;
use crate::util::HtmlTemplate;
use axum::Form;
use axum::{extract::State, http::StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use bcrypt::hash;
use validator::Validate;
/* #region PAGE HANDLERS */

pub async fn signup(State(state): State<AppState>) -> impl IntoResponse {
    let mut num = state.view_count.lock().unwrap();
    *num += 1;

    let template = SignupTemplate {
        view_count: *num,
        email_value: "".to_string(),
        email_error: None,
        password_error: None,
        confirm_password_error: None,
        username_error: None,
        username_value: "".to_string(),
    };
    HtmlTemplate(template)
}

pub async fn login(State(state): State<AppState>) -> impl IntoResponse {
    let mut num = state.view_count.lock().unwrap();
    *num += 1;

    let template = LoginTemplate {
        view_count: *num,
        email_error: None,
        password_error: None,
    };
    HtmlTemplate(template)
}

/* #endregion */

/* #region AUTHENTICATION HANDLERS */

pub async fn post_sign_up_handler(
    State(state): State<AppState>,
    Form(user_form): Form<AuthFormModel>,
) -> Response {
    // For debugging: Log the received form data.
    // Note: In a production environment, be cautious about logging sensitive data like passwords.
    tracing::info!(
        "Attempting to sign up with email: {}",
        user_form.email
    );
    // First, run the validators from the form model
    if let Err(e) = user_form.validate() {
        tracing::warn!("Sign-up form validation failed: {:?}", e);
        let mut num = state.view_count.lock().unwrap();
        *num += 1;

        let mut email_error = None;
        let mut password_error = None;
        let mut confirm_password_error = None;
        let mut username_error = None;

        // Map validation errors to the specific template fields
        for (field, errors) in e.field_errors() {
            let messages = errors
                .iter()
                .map(|error| error.message.as_ref().unwrap().to_string())
                .collect::<Vec<_>>()
                .join(", ");

            match field.as_ref() {
                "email" => email_error = Some(messages),
                "password" => password_error = Some(messages),
                "confirm_password" => confirm_password_error = Some(messages),
                "username" => username_error = Some(messages),
                _ => (),
            }
        }

        let template = SignupTemplate {
            view_count: *num,
            email_value: user_form.email, // Repopulate the form with the user's input
            email_error,
            password_error,
            confirm_password_error,
            username_error,
            username_value: user_form.username,
        };
        return HtmlTemplate(template).into_response();
    }

    // Check if a user with this email already exists
    match sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE email = ?)")
        .bind(&user_form.email)
        .fetch_one(&state.db_pool)
        .await
    {
        Ok(true) => {
            tracing::warn!(
                "Sign-up failed: user with email {} already exists.",
                user_form.email
            );
            // User exists, re-render form with an error
            let mut num = state.view_count.lock().unwrap();
            *num += 1;
            let template = SignupTemplate {
                view_count: *num,
                email_value: user_form.email,
                email_error: Some("A user with this email already exists.".to_string()),
                password_error: None,
                confirm_password_error: None,
                username_error: None, // Or check for username existence too
                username_value: user_form.username,
            };
            return HtmlTemplate(template).into_response();
        }
        Ok(false) => { // User does not exist, proceed
            tracing::info!(
                "New user email {} is unique. Proceeding with registration.",
                user_form.email
            );
        }
        Err(e) => {
            tracing::error!("Database error checking user existence: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    }

    // Hash the password
    let hashed_password = match hash(&user_form.password, bcrypt::DEFAULT_COST) {
        Ok(h) => {
            tracing::info!("Password hashed successfully.");
            h
        }
        Err(e) => {
            tracing::error!("Error hashing password: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    };

    // Insert the new user into the database
    tracing::info!("Inserting new user into the database.");
    if let Err(e) = sqlx::query("INSERT INTO users (email, hashed_password, username) VALUES (?, ?, ?)")
        .bind(&user_form.email)
        .bind(&hashed_password)
        .bind(&user_form.username)
        .execute(&state.db_pool)
        .await
    {
        tracing::error!("Failed to insert new user: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Could not create account.").into_response();
    }

    tracing::info!("New user created: {}", user_form.email);
    // On success, redirect to the login page
    Redirect::to("/login").into_response()
}


pub async fn post_log_in_handler(
    State(_state): State<AppState>,
    Form(user_form): Form<AuthFormModel>,
) -> Response {
    // TODO: This is a stub. Implement actual login logic.
    // 1. Validate form input (though we might rely on client-side validation for a simple case).
    // 2. Fetch user from the database by email.
    // 3. If user exists, verify the password using `bcrypt::verify`.
    // 4. If password is correct, create a session for the user.
    // 5. Redirect to a protected page or home page.
    // 6. If login fails, re-render the login form with an error message.
    tracing::info!(
        "Login attempt with email: {}", user_form.email
    );
    Redirect::to("/").into_response()
}

/* #endregion */
