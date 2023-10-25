use crate::errors::errors::Error;
use crate::models::model_user::CreateUser;

// Check if a string meets a minimum length requirement
pub fn check_constraint(
    value: &str,
    min_length: usize,
    error_msg: &'static str,
) -> Result<(), Error> {
    if value.len() < min_length {
        return Err(Error::CreateUserError("Invalid credentials.".to_string()));
    }
    Ok(())
}

// Check if a string contains an @ symbol
pub fn is_valid_email(email: &str) -> Result<(), Error> {
    if !email.contains('@') {
        return Err(Error::CreateUserError("Invalid email.".to_string()));
    }
    Ok(())
}

// Validate the payload for creating a new user
pub fn validate_payload(payload: &CreateUser) -> Result<(), Error> {
    check_constraint(
        &payload.password,
        8,
        "Password must be at least 8 characters long.",
    )?;
    check_constraint(
        &payload.username,
        3,
        "Username must be at least 3 characters long.",
    )?;
    check_constraint(
        &payload.display_name,
        3,
        "Display name must be at least 3 characters long.",
    )?;
    is_valid_email(&payload.email)?;
    Ok(())
}
