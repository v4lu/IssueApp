package main

import (
	"github.com/gin-gonic/gin"
	"github.com/valu/github-apis/internal/api/handlers"
	"github.com/valu/github-apis/internal/api/routes"
	"github.com/valu/github-apis/internal/services"
)

func main() {
	r := gin.Default()
	r.Group("/v1")

	githubService := services.NewGithubService()
	githubHandler := handlers.NewGithubHandler(*githubService)

	routes.GithubRoutes(r, *githubHandler)

	if err := r.Run(":9001"); err != nil {
		panic(err)
	}
}
