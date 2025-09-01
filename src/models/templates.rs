use askama::Template;

/* #region public page templates */

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate {
    pub view_count: u64,
}

#[derive(Template)]
#[template(path = "pages/notfound.html")]
pub struct NotFound {}

#[derive(Template)]
#[template(path = "pages/server-error.html")]
pub struct ServerError {}

#[derive(Template)]
#[template(path = "pages/about.html")]
pub struct AboutTemplate {
    pub view_count: u64,
}

#[derive(Template)]
#[template(path = "pages/contact.html")]
pub struct ContactTemplate {
    pub view_count: u64,
}

/* #endregion */

/* #region authentication page templates */
#[derive(Template)]
#[template(path = "pages/signup.html")]
pub struct SignupTemplate {
    pub view_count: u64,
    pub email_value: String,
    pub email_error: Option<String>,
    pub password_error: Option<String>,
    pub confirm_password_error: Option<String>,
    pub username_error: Option<String>,
    pub username_value: String,
}

#[derive(Template)]
#[template(path = "pages/login.html")]
pub struct LoginTemplate {
    pub view_count: u64,
    pub email_error: Option<String>,
    pub password_error: Option<String>,
}
/* #endregion */
