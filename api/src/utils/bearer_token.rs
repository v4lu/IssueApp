use crate::errors::CustomError;

pub fn extract_bearer_token(auth_header: &str) -> Result<&str, CustomError> {
    if !auth_header.starts_with("Bearer ") {
        return Err(CustomError::BadRequest);
    }
    Ok(&auth_header[7..])
}
