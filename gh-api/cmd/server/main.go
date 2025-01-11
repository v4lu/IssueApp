// @title           GitHub APIs
// @version         1.0
// @description     A REST API service for managing GitHub repositories and issues
// @termsOfService  http://swagger.io/terms/

// @contact.name   API Support
// @contact.url    http://www.swagger.io/support
// @contact.email  support@swagger.io

// @license.name  Apache 2.0
// @license.url   http://www.apache.org/licenses/LICENSE-2.0.html

// @host      localhost:9001
// @BasePath  /v1

// @securityDefinitions.apikey BearerAuth
// @in header
// @name Authorization
package main

import (
	"github.com/gin-gonic/gin"
	_ "github.com/valu/github-apis/docs"
	"github.com/valu/github-apis/internal/api/handlers"
	"github.com/valu/github-apis/internal/api/routes"
	"github.com/valu/github-apis/internal/services"
)

func main() {
	r := gin.Default()
	r.Group("/v1")

	githubService := services.NewGithubService()
	githubHandler := handlers.NewGithubHandler(*githubService)

	routes.GithubRoutes(r, githubHandler)

	if err := r.Run(":9001"); err != nil {
		panic(err)
	}
}
