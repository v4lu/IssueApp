package routes

import (
	"github.com/gin-gonic/gin"
	"github.com/valu/github-apis/internal/api/handlers"
	"github.com/valu/github-apis/internal/api/middlewares"
)

func GithubRoutes(r *gin.Engine, githubHandler handlers.GithubHandler) {

	r.Use(middlewares.AuthMiddleware())

	orgs := r.Group("/orgs/:org")
	{
		orgs.GET("/repos", githubHandler.GetRepositories)
	}

	repos := r.Group("/repos/:owner/:repo")
	{
		repos.GET("/issues", githubHandler.GetIssues)
		repos.POST("/issues", githubHandler.CreateIssue)
		repos.PATCH("/issues/:number", githubHandler.UpdateIssue)
	}

	r.GET("/repos", githubHandler.GetAccessibleRepos)
	r.GET("/organizations", githubHandler.GetAuthorizedOrganizations)
}
