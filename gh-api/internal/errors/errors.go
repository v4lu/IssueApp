package errors

type CustomError struct {
	Message string
}

func (e *CustomError) Error() string {
	return e.Message
}

func NewExternalServiceError(message string) *CustomError {
	return &CustomError{
		Message: "External Service Error: " + message,
	}
}
