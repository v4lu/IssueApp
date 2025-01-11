package handlers

import (
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
	"github.com/valu/github-apis/internal/errors"
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
		handleError(c, err)
		return
	}

	c.JSON(http.StatusOK, repos)
}

func handleError(c *gin.Context, err error) {
	if appErr, ok := err.(*errors.AppError); ok {
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

func (h *GithubHandler) GetIssues(c *gin.Context) {
	owner := c.Param("owner")
	repo := c.Param("repo")
	if owner == "" || repo == "" {
		handleError(c, errors.NewValidationError("owner and repository name are required"))
		return
	}

	token, err := getGithubToken(c)
	if err != nil {
		handleError(c, err)
		return
	}

	issues, err := h.githubService.GetRepositoryIssues(token, owner, repo)
	if err != nil {
		handleError(c, err)
		return
	}

	c.JSON(http.StatusOK, gin.H{"data": issues})
}

// CreateIssue godoc
// @Summary      Create a new issue
// @Description  Creates a new issue in the specified repository
// @Tags         issues
// @Accept       json
// @Produce      json
// @Param        owner path string true "Repository owner"
// @Param        repo path string true "Repository name"
// @Param        issue body models.CreateIssueRequest true "Issue details"
// @Success      201  {object}  models.Issue
// @Failure      400  {object}  errors.AppError
// @Failure      401  {object}  errors.AppError
// @Failure      404  {object}  errors.AppError
// @Failure      500  {object}  errors.AppError
// @Security     BearerAuth
// @Router       /repos/{owner}/{repo}/issues [post]
func (h *GithubHandler) CreateIssue(c *gin.Context) {
	owner := c.Param("owner")
	repo := c.Param("repo")
	if owner == "" || repo == "" {
		handleError(c, errors.NewValidationError("owner and repository name are required"))
		return
	}

	token, err := getGithubToken(c)
	if err != nil {
		handleError(c, err)
		return
	}

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
		handleError(c, errors.NewValidationError(err.Error()))
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

	issue, err := h.githubService.CreateIssue(token, &request)
	if err != nil {
		handleError(c, err)
		return
	}

	c.JSON(http.StatusCreated, gin.H{"data": issue})
}

func (h *GithubHandler) UpdateIssue(c *gin.Context) {
	owner := c.Param("owner")
	repo := c.Param("repo")
	issueNumber := c.Param("number")

	if owner == "" || repo == "" {
		handleError(c, errors.NewValidationError("owner and repository name are required"))
		return
	}

	token, err := getGithubToken(c)
	if err != nil {
		handleError(c, err)
		return
	}

	issueNum, err := strconv.ParseInt(issueNumber, 10, 64)
	if err != nil {
		handleError(c, errors.NewValidationError("invalid issue number"))
		return
	}

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
		handleError(c, errors.NewValidationError(err.Error()))
		return
	}

	request := models.UpdateIssueRequest{
		Owner:     owner,
		RepoName:  repo,
		IssueNum:  issueNum,
		Title:     updateRequest.Title,
		Body:      &updateRequest.Body,
		State:     &updateRequest.State,
		Assignees: updateRequest.Assignees,
		Labels:    updateRequest.Labels,
	}

	issue, err := h.githubService.UpdateIssue(token, &request)
	if err != nil {
		handleError(c, err)
		return
	}

	c.JSON(http.StatusOK, gin.H{"data": issue})
}

func (h *GithubHandler) DeleteIssue(c *gin.Context) {
	owner := c.Param("owner")
	repo := c.Param("repo")
	issueNumber := c.Param("number")

	if owner == "" || repo == "" {
		handleError(c, errors.NewValidationError("owner and repository name are required"))
		return
	}

	token, err := getGithubToken(c)
	if err != nil {
		handleError(c, err)
		return
	}

	issueNum, err := strconv.Atoi(issueNumber)
	if err != nil {
		handleError(c, errors.NewValidationError("invalid issue number"))
		return
	}

	err = h.githubService.DeleteIssue(token, owner, repo, issueNum)
	if err != nil {
		handleError(c, err)
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "Issue deleted successfully"})
}

func (h *GithubHandler) GetIssueByNumber(c *gin.Context) {
	owner := c.Param("owner")
	repo := c.Param("repo")
	issueNumber := c.Param("number")

	if owner == "" || repo == "" {
		handleError(c, errors.NewValidationError("owner and repository name are required"))
		return
	}

	token, err := getGithubToken(c)
	if err != nil {
		handleError(c, err)
		return
	}

	issueNum, err := strconv.Atoi(issueNumber)
	if err != nil {
		handleError(c, errors.NewValidationError("invalid issue number"))
		return
	}

	issue, err := h.githubService.GetIssueByNumber(token, owner, repo, issueNum)
	if err != nil {
		handleError(c, err)
		return
	}

	c.JSON(http.StatusOK, gin.H{"data": issue})
}

func (h *GithubHandler) GetAccessibleRepos(c *gin.Context) {
	token, err := getGithubToken(c)
	if err != nil {
		handleError(c, err)
		return
	}

	repos, err := h.githubService.GetAccessibleRepos(token)
	if err != nil {
		handleError(c, err)
		return
	}

	c.JSON(http.StatusOK, gin.H{"data": repos})
}

func (h *GithubHandler) GetAuthorizedOrganizations(c *gin.Context) {
	token, err := getGithubToken(c)
	if err != nil {
		handleError(c, err)
		return
	}

	orgs, err := h.githubService.GetAuthorizedOrganizations(token)
	if err != nil {
		handleError(c, err)
		return
	}

	c.JSON(http.StatusOK, gin.H{"data": orgs})
}

func getGithubToken(c *gin.Context) (string, error) {
	token, exists := c.Get("github_token")
	if !exists {
		return "", errors.NewAuthenticationError("GitHub token not found in context")
	}

	tokenStr, ok := token.(string)
	if !ok {
		return "", errors.NewAuthenticationError("Invalid GitHub token format")
	}

	return tokenStr, nil
}
