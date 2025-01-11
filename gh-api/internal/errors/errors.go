package errors

import (
	"fmt"
	"net/http"
)

type ErrorType string

const (
	ErrorTypeValidation     ErrorType = "VALIDATION_ERROR"
	ErrorTypeAuthentication ErrorType = "AUTHENTICATION_ERROR"
	ErrorTypeAuthorization  ErrorType = "AUTHORIZATION_ERROR"
	ErrorTypeNotFound       ErrorType = "NOT_FOUND"
	ErrorTypeGitHubAPI      ErrorType = "GITHUB_API_ERROR"
	ErrorTypeInternal       ErrorType = "INTERNAL_ERROR"
)

type AppError struct {
	Type       ErrorType
	Message    string
	HTTPStatus int
	Raw        error
}

func (e *AppError) Error() string {
	if e.Raw != nil {
		return fmt.Sprintf("%s: %s (caused by: %v)", e.Type, e.Message, e.Raw)
	}
	return fmt.Sprintf("%s: %s", e.Type, e.Message)
}

func NewValidationError(message string) *AppError {
	return &AppError{
		Type:       ErrorTypeValidation,
		Message:    message,
		HTTPStatus: http.StatusBadRequest,
	}
}

func NewAuthenticationError(message string) *AppError {
	return &AppError{
		Type:       ErrorTypeAuthentication,
		Message:    message,
		HTTPStatus: http.StatusUnauthorized,
	}
}

func NewGitHubError(err error, message string) *AppError {
	return &AppError{
		Type:       ErrorTypeGitHubAPI,
		Message:    message,
		HTTPStatus: http.StatusBadGateway,
		Raw:        err,
	}
}

func NewNotFoundError(message string) *AppError {
	return &AppError{
		Type:       ErrorTypeNotFound,
		Message:    message,
		HTTPStatus: http.StatusNotFound,
	}
}
