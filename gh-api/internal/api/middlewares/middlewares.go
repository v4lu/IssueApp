package middlewares

import (
	"net/http"
	"strings"

	"github.com/gin-gonic/gin"
	"github.com/valu/github-apis/internal/errors"
)

func AuthMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		authHeader := c.GetHeader("Authorization")
		if authHeader == "" {
			c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{
				"error": gin.H{
					"type":    errors.ErrorTypeAuthentication,
					"message": "Authorization header is required",
				},
			})
			return
		}

		parts := strings.Split(authHeader, " ")
		if len(parts) != 2 || !strings.EqualFold(parts[0], "Bearer") {
			c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{
				"error": gin.H{
					"type":    errors.ErrorTypeAuthentication,
					"message": "Invalid authorization format. Use 'Bearer <token>'",
				},
			})
			return
		}

		token := strings.TrimSpace(parts[1])
		if token == "" {
			c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{
				"error": gin.H{
					"type":    errors.ErrorTypeAuthentication,
					"message": "Token cannot be empty",
				},
			})
			return
		}

		c.Set("github_token", token)
		c.Next()
	}
}

func ErrorMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Next()

		if len(c.Errors) > 0 {
			err := c.Errors.Last()
			if appErr, ok := err.Err.(*errors.AppError); ok {
				c.JSON(appErr.HTTPStatus, gin.H{
					"error": gin.H{
						"type":    appErr.Type,
						"message": appErr.Message,
					},
				})
				return
			}

			c.JSON(http.StatusInternalServerError, gin.H{
				"error": gin.H{
					"type":    errors.ErrorTypeInternal,
					"message": "An unexpected error occurred",
				},
			})
		}
	}
}
