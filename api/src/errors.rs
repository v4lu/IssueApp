use crate::models::error::ErrorResponse;
use actix_web::{HttpResponse, ResponseError};
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum CustomError {
    #[error("Bad Request")]
    BadRequest,
    #[error("{0} not found")]
    NotFound(String),
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Database Error: {0}")]
    DatabaseError(String),
    #[error("Config Error: {0}")]
    ConfigError(String),
    #[error("Conflict: {0}, {1}")]
    Conflict(String, String),
    #[error("Not Strong Password")]
    NotStrongPassword,
    #[error("Invalid Confirmation Code")]
    InvalidConfirmationCode,
    #[error("Code Expired")]
    CodeExpired,
    #[error("Account Already Confirmed")]
    AccountAlreadyConfirmed,
    #[error("Account Not Found")]
    AccountNotFound,
    #[error("Invalid Credentials")]
    InvalidCredentials,
    #[error("Account Not Verified")]
    AccountNotVerified,
    #[error("Account Disabled")]
    AccountDisabled,
    #[error("Too Many Failed Attempts")]
    TooManyAttempts,
    #[error("{0}")]
    InvalidToken(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Token expired")]
    TokenExpired,
    #[error("Validation Error")]
    ValidationError(ValidationErrors),
    #[error("External Service Error: {0}")]
    ExternalServiceError(String),
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            // Resource Errors - 404
            CustomError::NotFound(resource) => HttpResponse::NotFound().json(ErrorResponse {
                error: format!("{} not found", resource),
                code: "RES_001".to_string(),
                message: "The requested resource could not be found".to_string(),
                validation_errors: None,

            }),

            // Server Errors - 500
            CustomError::InternalServerError => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Internal server error".to_string(),
                    code: "SRV_001".to_string(),
                    message: "An unexpected error occurred".to_string(),
                    validation_errors: None,

                })
            }

            // Default Bad Request 400
            CustomError::BadRequest => HttpResponse::BadRequest().json(ErrorResponse {
                error: "Bad Request".to_string(),
                code: "BAD_001".to_string(),
                message: "Bad request".to_string(),
                validation_errors: None,

            }),

            // Database Errors - 500
            CustomError::DatabaseError(message) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database error".to_string(),
                    code: "DB_001".to_string(),
                    message: message.to_string(),
                    validation_errors: None,

                })
            }

            // Config Errors - 500
            CustomError::ConfigError(message) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Config error".to_string(),
                    code: "CFG_001".to_string(),
                    message: message.to_string(),
                    validation_errors: None,

                })
            }


            // Conflict Errors - 409
            CustomError::Conflict(message, code) => HttpResponse::Conflict().json(ErrorResponse {
                error: "Conflict".to_string(),
                code: code.to_string(),
                message: message.to_string(),
                validation_errors: None,

            }),

            // Not Strong Password - 400
            CustomError::NotStrongPassword => HttpResponse::BadRequest().json(ErrorResponse {
                error: "Not Strong Password".to_string(),
                code: "PWD_001".to_string(),
                message: "Password must be at least 8 characters long and contain at least one uppercase letter, one lowercase letter, one number, and one special character".to_string(),
                validation_errors: None,

            }),

            // Invalid Confirmation Code - 400
            CustomError::InvalidConfirmationCode => HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid confirmation code".to_string(),
                code: "CON_002".to_string(),
                message: "The provided confirmation code is invalid".to_string(),
                validation_errors: None,

            }),

            // Code Expired - 400
            CustomError::CodeExpired => HttpResponse::BadRequest().json(ErrorResponse {
                error: "Code expired".to_string(),
                code: "CON_003".to_string(),
                message: "The confirmation code has expired".to_string(),
                validation_errors: None,

            }),

            // Account Already Confirmed - 400
            CustomError::AccountAlreadyConfirmed => HttpResponse::BadRequest().json(ErrorResponse {
                error: "Account already confirmed".to_string(),
                code: "CON_004".to_string(),
                message: "This account has already been confirmed".to_string(),
                validation_errors: None,

            }),

            // Account Not Found - 404
            CustomError::AccountNotFound => HttpResponse::NotFound().json(ErrorResponse {
                error: "Account not found".to_string(),
                code: "CON_005".to_string(),
                message: "No account found with the provided email".to_string(),
                validation_errors: None,

            }),

            // Invalid Credentials - 401
            CustomError::InvalidCredentials => HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Invalid credentials".to_string(),
                code: "AUTH_001".to_string(),
                message: "The provided email or password is incorrect".to_string(),
                validation_errors: None,

            }),

            // Account Not Verified - 403
            CustomError::AccountNotVerified => HttpResponse::Forbidden().json(ErrorResponse {
                error: "Account not verified".to_string(),
                code: "AUTH_002".to_string(),
                message: "Please verify your email address before logging in".to_string(),
                validation_errors: None,

            }),

            // Account Disabled - 403
            CustomError::AccountDisabled => HttpResponse::Forbidden().json(ErrorResponse {
                error: "Account disabled".to_string(),
                code: "AUTH_003".to_string(),
                message: "Your account has been disabled. Please contact support".to_string(),
                validation_errors: None,

            }),

            // Too Many Failed Attempts - 429
            CustomError::TooManyAttempts => HttpResponse::TooManyRequests().json(ErrorResponse {
                error: "Too many attempts".to_string(),
                code: "AUTH_004".to_string(),
                message: "Too many failed login attempts. Please try again later".to_string(),
                validation_errors: None,

            }),

            // Invalid Token - 401
            CustomError::InvalidToken(message) => HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Invalid token".to_string(),
                code: "AUTH_005".to_string(),
                message: message.to_string(),
                validation_errors: None,

            }),

            // Unauthorized - 401
            CustomError::Unauthorized => HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Unauthorized".to_string(),
                code: "AUTH_006".to_string(),
                message: "You are not authorized to access this resource".to_string(),
                validation_errors: None,

            }),

            // Forbidden - 403
            CustomError::Forbidden => HttpResponse::Forbidden().json(ErrorResponse {
                error: "Forbidden".to_string(),
                code: "AUTH_007".to_string(),
                message: "You do not have permission to access this resource".to_string(),
                validation_errors: None,

            }),

            // Token Expired - 401
            CustomError::TokenExpired => HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Token expired".to_string(),
                code: "AUTH_008".to_string(),
                message: "The provided token has expired".to_string(),
                validation_errors: None,
            }),

            // Validation Error - 422
            CustomError::ValidationError(errors) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "".to_string(),
                    code: "VAL_001".to_string(),
                    message: format!("{:?}", errors),
                    validation_errors: Some(serde_json::to_value(errors).unwrap()),
                })
            }

            // External Service Error - 500
            CustomError::ExternalServiceError(message) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "External service error".to_string(),
                    code: "EXT_001".to_string(),
                    message: message.to_string(),
                    validation_errors: None,
                })
            }
        }
    }
}
