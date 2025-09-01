use serde::Deserialize;
use validator::{Validate, ValidationError};

fn validate_password(password: &str) -> bool {
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let is_long_enough = password.len() >= 8;

    has_lowercase && has_uppercase && has_digit && is_long_enough
}

fn password_is_valid(password: &str) -> Result<(), ValidationError> {
    if !validate_password(password) {
        // Return an error if validation fails
        Err(ValidationError::new(
            "Invalid Password",
        ))
    } else {
        Ok(())
    }
}

#[derive(Deserialize, Validate)]
pub struct AuthFormModel {
    #[validate(email(message = "Please enter a valid email."))]
    pub email: String,
    #[validate(
        custom(function = "password_is_valid", message = "Password must be at least 8 characters long and include an uppercase letter, a lowercase letter, and a number.")
    )]
    pub password: String,
    #[validate(must_match(other = "password", message = "The passwords do not match."))]
    pub confirm_password: String,
    #[validate(length(min = 3, message = "Username must be at least 3 characters long."))]
    pub username: String,
}
