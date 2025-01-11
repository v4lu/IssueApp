package services

import (
	"context"
	"fmt"

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
	ctx := context.Background()
	ts := oauth2.StaticTokenSource(
		&oauth2.Token{AccessToken: accessToken},
	)
	tc := oauth2.NewClient(ctx, ts)
	client := github.NewClient(tc)

	opt := &github.RepositoryListByOrgOptions{
		ListOptions: github.ListOptions{PerPage: 100},
	}

	fmt.Println("Fetching repos for org:", orgName)
	var allRepos []models.Repository

	for {
		repos, resp, err := client.Repositories.ListByOrg(ctx, orgName, opt)
		if err != nil {
			return nil, errors.NewExternalServiceError(err.Error())
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
	ctx := context.Background()
	ts := oauth2.StaticTokenSource(
		&oauth2.Token{AccessToken: accessToken},
	)
	tc := oauth2.NewClient(ctx, ts)
	client := github.NewClient(tc)

	repo, _, err := client.Repositories.Get(ctx, owner, repoName)
	if err != nil {
		return nil, errors.NewExternalServiceError(err.Error())
	}

	result := convertToRepository(repo)
	return &result, nil
}

func (s *GithubService) GetRepositoryIssues(accessToken, owner, repoName string) ([]models.Issue, error) {
	ctx := context.Background()
	ts := oauth2.StaticTokenSource(
		&oauth2.Token{AccessToken: accessToken},
	)
	tc := oauth2.NewClient(ctx, ts)
	client := github.NewClient(tc)

	opt := &github.IssueListByRepoOptions{
		State:       "all",
		ListOptions: github.ListOptions{PerPage: 100},
	}
	var allIssues []models.Issue

	for {
		issues, resp, err := client.Issues.ListByRepo(ctx, owner, repoName, opt)
		if err != nil {
			return nil, errors.NewExternalServiceError(err.Error())
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
	ctx := context.Background()
	ts := oauth2.StaticTokenSource(
		&oauth2.Token{AccessToken: accessToken},
	)
	tc := oauth2.NewClient(ctx, ts)
	client := github.NewClient(tc)

	issue, _, err := client.Issues.Get(ctx, owner, repoName, issueNumber)
	if err != nil {
		return nil, errors.NewExternalServiceError(err.Error())
	}

	result := convertToIssue(issue)
	return &result, nil
}

func (s *GithubService) UpdateIssue(accessToken string, request *models.UpdateIssueRequest) (*models.Issue, error) {
	ctx := context.Background()
	ts := oauth2.StaticTokenSource(
		&oauth2.Token{AccessToken: accessToken},
	)
	tc := oauth2.NewClient(ctx, ts)
	client := github.NewClient(tc)

	req := &github.IssueRequest{
		Title:     &request.Title,
		Body:      request.Body,
		State:     request.State,
		Assignees: &request.Assignees,
		Labels:    &request.Labels,
	}

	issue, _, err := client.Issues.Edit(ctx, request.Owner, request.RepoName, int(request.IssueNum), req)
	if err != nil {
		return nil, errors.NewExternalServiceError(err.Error())
	}

	result := convertToIssue(issue)
	return &result, nil
}

func (s *GithubService) CreateIssue(accessToken string, request *models.CreateIssueRequest) (*models.Issue, error) {
	ctx := context.Background()
	ts := oauth2.StaticTokenSource(
		&oauth2.Token{AccessToken: accessToken},
	)
	tc := oauth2.NewClient(ctx, ts)
	client := github.NewClient(tc)

	req := &github.IssueRequest{
		Title:     &request.Title,
		Body:      request.Body,
		Assignees: &request.Assignees,
		Labels:    &request.Labels,
	}

	issue, _, err := client.Issues.Create(ctx, request.Owner, request.RepoName, req)
	if err != nil {
		return nil, errors.NewExternalServiceError(err.Error())
	}

	result := convertToIssue(issue)
	return &result, nil
}

func (s *GithubService) GetAccessibleRepos(accessToken string) ([]models.Repository, error) {
	ctx := context.Background()
	ts := oauth2.StaticTokenSource(
		&oauth2.Token{AccessToken: accessToken},
	)
	tc := oauth2.NewClient(ctx, ts)
	client := github.NewClient(tc)
	opt := &github.RepositoryListOptions{
		ListOptions: github.ListOptions{PerPage: 100},
	}
	var allRepos []models.Repository

	for {
		repos, resp, err := client.Repositories.List(ctx, "", opt)
		if err != nil {
			return nil, errors.NewExternalServiceError(err.Error())
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
	ctx := context.Background()
	ts := oauth2.StaticTokenSource(
		&oauth2.Token{AccessToken: accessToken},
	)
	tc := oauth2.NewClient(ctx, ts)
	client := github.NewClient(tc)

	opt := &github.ListOptions{PerPage: 100}
	var allOrgs []models.Organization

	for {
		orgs, resp, err := client.Organizations.List(ctx, "", opt)
		if err != nil {
			return nil, errors.NewExternalServiceError(err.Error())
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
