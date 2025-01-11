package handlers

import (
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
	"github.com/valu/github-apis/internal/models"
	"github.com/valu/github-apis/internal/services"
)

type GithubHandler struct {
	githubService services.GithubService
}

func NewGithubHandler(githubService services.GithubService) *GithubHandler {
	return &GithubHandler{
		githubService: githubService,
	}
}

func (h *GithubHandler) GetRepositories(c *gin.Context) {
	orgName := c.Param("org")
	token := c.MustGet("github_token").(string)

	repos, err := h.githubService.GetOrgRepos(orgName, token)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, repos)
}

func (h *GithubHandler) GetIssues(c *gin.Context) {
	owner := c.Param("owner")
	repo := c.Param("repo")
	token := c.MustGet("github_token").(string)

	issues, err := h.githubService.GetRepositoryIssues(token, owner, repo)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, issues)
}

func (h *GithubHandler) CreateIssue(c *gin.Context) {
	owner := c.Param("owner")
	repo := c.Param("repo")
	token := c.MustGet("github_token").(string)

	var issueRequest struct {
		Title     string   `json:"title" binding:"required"`
		Body      string   `json:"body"`
		Assignees []string `json:"assignees"`
		Labels    []string `json:"labels"`
	}

	if issueRequest.Assignees == nil {
		issueRequest.Assignees = []string{}
	}
	if issueRequest.Labels == nil {
		issueRequest.Labels = []string{}
	}

	if err := c.ShouldBindJSON(&issueRequest); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	request := models.CreateIssueRequest{
		Owner:     owner,
		RepoName:  repo,
		Title:     issueRequest.Title,
		Body:      &issueRequest.Body,
		Assignees: issueRequest.Assignees,
		Labels:    issueRequest.Labels,
	}

	issue, err := h.githubService.CreateIssue(
		token,
		&request,
	)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusCreated, issue)
}

func (h *GithubHandler) UpdateIssue(c *gin.Context) {
	owner := c.Param("owner")
	repo := c.Param("repo")
	issueNumber := c.Param("number")
	token := c.MustGet("github_token").(string)

	var updateRequest struct {
		Title     string   `json:"title"`
		Body      string   `json:"body"`
		State     string   `json:"state"`
		Assignees []string `json:"assignees"`
		Labels    []string `json:"labels"`
	}

	if updateRequest.Assignees == nil {
		updateRequest.Assignees = []string{}
	}
	if updateRequest.Labels == nil {
		updateRequest.Labels = []string{}
	}

	if err := c.ShouldBindJSON(&updateRequest); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	issueNum, err := strconv.ParseInt(issueNumber, 10, 64)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid issue number"})
		return
	}

	request := models.UpdateIssueRequest{
		Title:     updateRequest.Title,
		Body:      &updateRequest.Body,
		State:     &updateRequest.State,
		Assignees: updateRequest.Assignees,
		Labels:    updateRequest.Labels,
		Owner:     owner,
		RepoName:  repo,
		IssueNum:  issueNum,
	}
	issue, err := h.githubService.UpdateIssue(
		token,
		&request,
	)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, issue)
}

func (h *GithubHandler) GetAccessibleRepos(c *gin.Context) {
	token := c.MustGet("github_token").(string)

	repos, err := h.githubService.GetAccessibleRepos(token)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, repos)
}

func (h *GithubHandler) GetAuthorizedOrganizations(c *gin.Context) {
	token := c.MustGet("github_token").(string)

	orgs, err := h.githubService.GetAuthorizedOrganizations(token)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, orgs)
}
