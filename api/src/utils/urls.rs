use actix_web::HttpRequest;

use crate::errors::CustomError;

pub fn extract_query_params_code(
    req: HttpRequest,
    wanted_param: &str,
) -> Result<String, CustomError> {
    let code = req
        .query_string()
        .split('&')
        .find(|param| param.starts_with(wanted_param))
        .map(|param| param.split('=').collect::<Vec<&str>>()[1].to_string())
        .ok_or(CustomError::InternalServerError)?;

    return Ok(code);
}
