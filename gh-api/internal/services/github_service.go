package services

import (
	"context"
	"fmt"
	"net/http"

	"github.com/google/go-github/v45/github"
	"github.com/valu/github-apis/internal/errors"
	"github.com/valu/github-apis/internal/models"
	"golang.org/x/oauth2"
)

type GithubService struct {
}

func NewGithubService() *GithubService {

	return &GithubService{}
}

func (s *GithubService) GetOrgRepos(orgName, accessToken string) ([]models.Repository, error) {
	if orgName == "" {
		return nil, errors.NewValidationError("organization name is required")
	}

	ctx := context.Background()
	client, err := s.createGitHubClient(accessToken)
	if err != nil {
		return nil, err
	}

	opt := &github.RepositoryListByOrgOptions{
		ListOptions: github.ListOptions{PerPage: 100},
	}

	var allRepos []models.Repository

	for {
		repos, resp, err := client.Repositories.ListByOrg(ctx, orgName, opt)
		if err != nil {
			if _, ok := err.(*github.RateLimitError); ok {
				return nil, errors.NewGitHubError(err, "GitHub API rate limit exceeded")
			}
			if resp != nil && resp.StatusCode == http.StatusNotFound {
				return nil, errors.NewNotFoundError(fmt.Sprintf("organization %s not found", orgName))
			}
			if resp != nil && resp.StatusCode == http.StatusUnauthorized {
				return nil, errors.NewAuthenticationError("invalid or expired GitHub token")
			}
			return nil, errors.NewGitHubError(err, "failed to fetch repositories")
		}

		for _, repo := range repos {
			allRepos = append(allRepos, convertToRepository(repo))
		}

		if resp.NextPage == 0 {
			break
		}
		opt.Page = resp.NextPage
	}

	return allRepos, nil
}

func (s *GithubService) GetRepository(accessToken, owner, repoName string) (*models.Repository, error) {
	if owner == "" || repoName == "" {
		return nil, errors.NewValidationError("owner and repository name are required")
	}

	client, err := s.createGitHubClient(accessToken)
	if err != nil {
		return nil, err
	}

	ctx := context.Background()
	repo, resp, err := client.Repositories.Get(ctx, owner, repoName)
	if err != nil {
		if _, ok := err.(*github.RateLimitError); ok {
			return nil, errors.NewGitHubError(err, "GitHub API rate limit exceeded")
		}
		if resp != nil && resp.StatusCode == http.StatusNotFound {
			return nil, errors.NewNotFoundError(fmt.Sprintf("repository %s/%s not found", owner, repoName))
		}
		if resp != nil && resp.StatusCode == http.StatusUnauthorized {
			return nil, errors.NewAuthenticationError("invalid or expired GitHub token")
		}
		return nil, errors.NewGitHubError(err, "failed to fetch repository")
	}

	result := convertToRepository(repo)
	return &result, nil
}

func (s *GithubService) GetRepositoryIssues(accessToken, owner, repoName string) ([]models.Issue, error) {
	if owner == "" || repoName == "" {
		return nil, errors.NewValidationError("owner and repository name are required")
	}

	client, err := s.createGitHubClient(accessToken)
	if err != nil {
		return nil, err
	}

	ctx := context.Background()
	opt := &github.IssueListByRepoOptions{
		State:       "all",
		ListOptions: github.ListOptions{PerPage: 100},
	}
	var allIssues []models.Issue

	for {
		issues, resp, err := client.Issues.ListByRepo(ctx, owner, repoName, opt)
		if err != nil {
			if _, ok := err.(*github.RateLimitError); ok {
				return nil, errors.NewGitHubError(err, "GitHub API rate limit exceeded")
			}
			if resp != nil && resp.StatusCode == http.StatusNotFound {
				return nil, errors.NewNotFoundError(fmt.Sprintf("repository %s/%s not found", owner, repoName))
			}
			if resp != nil && resp.StatusCode == http.StatusUnauthorized {
				return nil, errors.NewAuthenticationError("invalid or expired GitHub token")
			}
			return nil, errors.NewGitHubError(err, "failed to fetch issues")
		}

		for _, issue := range issues {
			allIssues = append(allIssues, convertToIssue(issue))
		}

		if resp.NextPage == 0 {
			break
		}
		opt.Page = resp.NextPage
	}

	return allIssues, nil
}

func (s *GithubService) GetIssue(accessToken, owner, repoName string, issueNumber int) (*models.Issue, error) {
	if owner == "" || repoName == "" {
		return nil, errors.NewValidationError("owner and repository name are required")
	}
	if issueNumber <= 0 {
		return nil, errors.NewValidationError("invalid issue number")
	}

	client, err := s.createGitHubClient(accessToken)
	if err != nil {
		return nil, err
	}

	ctx := context.Background()
	issue, resp, err := client.Issues.Get(ctx, owner, repoName, issueNumber)
	if err != nil {
		if _, ok := err.(*github.RateLimitError); ok {
			return nil, errors.NewGitHubError(err, "GitHub API rate limit exceeded")
		}
		if resp != nil && resp.StatusCode == http.StatusNotFound {
			return nil, errors.NewNotFoundError(fmt.Sprintf("issue #%d not found in %s/%s", issueNumber, owner, repoName))
		}
		if resp != nil && resp.StatusCode == http.StatusUnauthorized {
			return nil, errors.NewAuthenticationError("invalid or expired GitHub token")
		}
		return nil, errors.NewGitHubError(err, "failed to fetch issue")
	}

	result := convertToIssue(issue)
	return &result, nil
}

func (s *GithubService) UpdateIssue(accessToken string, request *models.UpdateIssueRequest) (*models.Issue, error) {
	if request == nil {
		return nil, errors.NewValidationError("update request cannot be nil")
	}
	if request.Owner == "" || request.RepoName == "" {
		return nil, errors.NewValidationError("owner and repository name are required")
	}
	if request.IssueNum <= 0 {
		return nil, errors.NewValidationError("invalid issue number")
	}

	client, err := s.createGitHubClient(accessToken)
	if err != nil {
		return nil, err
	}

	req := &github.IssueRequest{
		Title:     &request.Title,
		Body:      request.Body,
		State:     request.State,
		Assignees: &request.Assignees,
		Labels:    &request.Labels,
	}

	ctx := context.Background()
	issue, resp, err := client.Issues.Edit(ctx, request.Owner, request.RepoName, int(request.IssueNum), req)
	if err != nil {
		if _, ok := err.(*github.RateLimitError); ok {
			return nil, errors.NewGitHubError(err, "GitHub API rate limit exceeded")
		}
		if resp != nil && resp.StatusCode == http.StatusNotFound {
			return nil, errors.NewNotFoundError(fmt.Sprintf("issue #%d not found in %s/%s", request.IssueNum, request.Owner, request.RepoName))
		}
		if resp != nil && resp.StatusCode == http.StatusUnauthorized {
			return nil, errors.NewAuthenticationError("invalid or expired GitHub token")
		}
		return nil, errors.NewGitHubError(err, "failed to update issue")
	}

	result := convertToIssue(issue)
	return &result, nil
}

func (s *GithubService) CreateIssue(accessToken string, request *models.CreateIssueRequest) (*models.Issue, error) {
	if request == nil {
		return nil, errors.NewValidationError("create request cannot be nil")
	}
	if request.Owner == "" || request.RepoName == "" {
		return nil, errors.NewValidationError("owner and repository name are required")
	}
	if request.Title == "" {
		return nil, errors.NewValidationError("issue title is required")
	}

	client, err := s.createGitHubClient(accessToken)
	if err != nil {
		return nil, err
	}

	req := &github.IssueRequest{
		Title:     &request.Title,
		Body:      request.Body,
		Assignees: &request.Assignees,
		Labels:    &request.Labels,
	}

	ctx := context.Background()
	issue, resp, err := client.Issues.Create(ctx, request.Owner, request.RepoName, req)
	if err != nil {
		if _, ok := err.(*github.RateLimitError); ok {
			return nil, errors.NewGitHubError(err, "GitHub API rate limit exceeded")
		}
		if resp != nil && resp.StatusCode == http.StatusNotFound {
			return nil, errors.NewNotFoundError(fmt.Sprintf("repository %s/%s not found", request.Owner, request.RepoName))
		}
		if resp != nil && resp.StatusCode == http.StatusUnauthorized {
			return nil, errors.NewAuthenticationError("invalid or expired GitHub token")
		}
		fmt.Println(err)
		return nil, errors.NewGitHubError(err, "failed to create issue")
	}

	result := convertToIssue(issue)
	return &result, nil
}

func (s *GithubService) DeleteIssue(accessToken string, owner, repoName string, issueNumber int) error {
	if owner == "" || repoName == "" {
		return errors.NewValidationError("owner and repository name are required")
	}
	if issueNumber <= 0 {
		return errors.NewValidationError("invalid issue number")
	}

	client, err := s.createGitHubClient(accessToken)
	if err != nil {
		return err
	}

	ctx := context.Background()
	req := &github.IssueRequest{
		State: github.String("closed"),
	}
	_, resp, err := client.Issues.Edit(ctx, owner, repoName, issueNumber, req)
	if err != nil {
		if _, ok := err.(*github.RateLimitError); ok {
			return errors.NewGitHubError(err, "GitHub API rate limit exceeded")
		}
		if resp != nil && resp.StatusCode == http.StatusNotFound {
			return errors.NewNotFoundError(fmt.Sprintf("issue #%d not found in %s/%s", issueNumber, owner, repoName))
		}
		if resp != nil && resp.StatusCode == http.StatusUnauthorized {
			return errors.NewAuthenticationError("invalid or expired GitHub token")
		}
		return errors.NewGitHubError(err, "failed to close issue")
	}

	return nil
}

func (s *GithubService) GetIssueByNumber(accessToken, owner, repoName string, issueNumber int) (*models.Issue, error) {
	if owner == "" || repoName == "" {
		return nil, errors.NewValidationError("owner and repository name are required")
	}
	if issueNumber <= 0 {
		return nil, errors.NewValidationError("invalid issue number")
	}

	client, err := s.createGitHubClient(accessToken)
	if err != nil {
		return nil, err
	}

	ctx := context.Background()
	issue, resp, err := client.Issues.Get(ctx, owner, repoName, issueNumber)
	if err != nil {
		if _, ok := err.(*github.RateLimitError); ok {
			return nil, errors.NewGitHubError(err, "GitHub API rate limit exceeded")
		}
		if resp != nil && resp.StatusCode == http.StatusNotFound {
			return nil, errors.NewNotFoundError(fmt.Sprintf("issue #%d not found in %s/%s", issueNumber, owner, repoName))
		}
		if resp != nil && resp.StatusCode == http.StatusUnauthorized {
			return nil, errors.NewAuthenticationError("invalid or expired GitHub token")
		}
		return nil, errors.NewGitHubError(err, "failed to fetch issue")
	}

	result := convertToIssue(issue)
	return &result, nil
}

func (s *GithubService) GetAccessibleRepos(accessToken string) ([]models.Repository, error) {
	client, err := s.createGitHubClient(accessToken)
	if err != nil {
		return nil, err
	}

	ctx := context.Background()
	opt := &github.RepositoryListOptions{
		ListOptions: github.ListOptions{PerPage: 100},
	}
	var allRepos []models.Repository

	for {
		repos, resp, err := client.Repositories.List(ctx, "", opt)
		if err != nil {
			if _, ok := err.(*github.RateLimitError); ok {
				return nil, errors.NewGitHubError(err, "GitHub API rate limit exceeded")
			}
			if resp != nil && resp.StatusCode == http.StatusUnauthorized {
				return nil, errors.NewAuthenticationError("invalid or expired GitHub token")
			}
			return nil, errors.NewGitHubError(err, "failed to fetch accessible repositories")
		}

		for _, repo := range repos {
			allRepos = append(allRepos, convertToRepository(repo))
		}

		if resp.NextPage == 0 {
			break
		}
		opt.Page = resp.NextPage
	}

	return allRepos, nil
}

func (s *GithubService) GetAuthorizedOrganizations(accessToken string) ([]models.Organization, error) {
	client, err := s.createGitHubClient(accessToken)
	if err != nil {
		return nil, err
	}

	ctx := context.Background()
	opt := &github.ListOptions{PerPage: 100}
	var allOrgs []models.Organization

	for {
		orgs, resp, err := client.Organizations.List(ctx, "", opt)
		if err != nil {
			if _, ok := err.(*github.RateLimitError); ok {
				return nil, errors.NewGitHubError(err, "GitHub API rate limit exceeded")
			}
			if resp != nil && resp.StatusCode == http.StatusUnauthorized {
				return nil, errors.NewAuthenticationError("invalid or expired GitHub token")
			}
			return nil, errors.NewGitHubError(err, "failed to fetch organizations")
		}

		for _, org := range orgs {
			allOrgs = append(allOrgs, convertToOrganization(org))
		}

		if resp.NextPage == 0 {
			break
		}
		opt.Page = resp.NextPage
	}

	return allOrgs, nil
}

func (s *GithubService) createGitHubClient(accessToken string) (*github.Client, error) {
	if accessToken == "" {
		return nil, errors.NewAuthenticationError("GitHub token is required")
	}

	ctx := context.Background()
	ts := oauth2.StaticTokenSource(
		&oauth2.Token{AccessToken: accessToken},
	)
	tc := oauth2.NewClient(ctx, ts)
	return github.NewClient(tc), nil
}

func convertToRepository(repo *github.Repository) models.Repository {
	return models.Repository{
		ID:          repo.GetID(),
		Name:        repo.GetName(),
		FullName:    repo.GetFullName(),
		Description: repo.GetDescription(),
		Private:     repo.GetPrivate(),
		Fork:        repo.GetFork(),
		CreatedAt:   repo.GetCreatedAt().Time,
		UpdatedAt:   repo.GetUpdatedAt().Time,
		PushedAt:    repo.GetPushedAt().Time,
	}
}

func convertToIssue(issue *github.Issue) models.Issue {
	assignees := make([]string, 0)
	for _, assignee := range issue.Assignees {
		assignees = append(assignees, assignee.GetLogin())
	}

	labels := make([]string, 0)
	for _, label := range issue.Labels {
		labels = append(labels, label.GetName())
	}

	return models.Issue{
		Number:    issue.GetNumber(),
		Title:     issue.GetTitle(),
		Body:      issue.GetBody(),
		State:     issue.GetState(),
		CreatedAt: issue.GetCreatedAt(),
		UpdatedAt: issue.GetUpdatedAt(),
		Assignees: assignees,
		Labels:    labels,
	}
}

func convertToOrganization(org *github.Organization) models.Organization {
	return models.Organization{
		ID:          org.GetID(),
		Login:       org.GetLogin(),
		Name:        org.GetName(),
		Description: org.GetDescription(),
	}

}
