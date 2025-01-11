package routes

import (
	"github.com/gin-gonic/gin"
	swaggerFiles "github.com/swaggo/files"
	ginSwagger "github.com/swaggo/gin-swagger"
	"github.com/valu/github-apis/internal/api/handlers"
	"github.com/valu/github-apis/internal/api/middlewares"
)

func GithubRoutes(r *gin.Engine, githubHandler *handlers.GithubHandler) {
	if r == nil {
		panic("router cannot be nil")
	}
	if githubHandler == nil {
		panic("githubHandler cannot be nil")
	}

	r.GET("/swagger/*any", ginSwagger.WrapHandler(swaggerFiles.Handler))

	r.Use(gin.Recovery())
	r.Use(middlewares.ErrorMiddleware())
	r.Use(middlewares.AuthMiddleware())

	v1 := r.Group("/v1")
	{
		orgs := v1.Group("/orgs/:org")
		{
			orgs.GET("/repos", githubHandler.GetRepositories)
		}

		repos := v1.Group("/repos/:owner/:repo")
		{
			repos.GET("/issues", githubHandler.GetIssues)
			repos.POST("/issues", githubHandler.CreateIssue)
			repos.GET("/issues/:number", githubHandler.GetIssueByNumber)
			repos.PATCH("/issues/:number", githubHandler.UpdateIssue)
			repos.DELETE("/issues/:number", githubHandler.DeleteIssue)
		}

		v1.GET("/repos", githubHandler.GetAccessibleRepos)
		v1.GET("/organizations", githubHandler.GetAuthorizedOrganizations)
	}
}
