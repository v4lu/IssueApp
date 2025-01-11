package models

import "time"

type Repository struct {
	ID          int64     `json:"id"`
	Name        string    `json:"name"`
	FullName    string    `json:"full_name"`
	Description string    `json:"description"`
	Private     bool      `json:"private"`
	Fork        bool      `json:"fork"`
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
	PushedAt    time.Time `json:"pushed_at"`
}

type Issue struct {
	Number    int       `json:"number"`
	Title     string    `json:"title"`
	Body      string    `json:"body"`
	State     string    `json:"state"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
	Assignees []string  `json:"assignees"`
	Labels    []string  `json:"labels"`
}

type Organization struct {
	ID          int64  `json:"id"`
	Login       string `json:"login"`
	Name        string `json:"name"`
	Description string `json:"description"`
}

type CreateIssueRequest struct {
	Owner     string   `json:"owner"`
	RepoName  string   `json:"repoName"`
	Title     string   `json:"title"`
	Body      *string  `json:"body"`
	Assignees []string `json:"assignees"`
	Labels    []string `json:"labels"`
}

type UpdateIssueRequest struct {
	Owner     string   `json:"owner"`
	RepoName  string   `json:"repoName"`
	IssueNum  int64    `json:"issueNum"`
	Title     string   `json:"title"`
	Body      *string  `json:"body"`
	State     *string  `json:"state"`
	Assignees []string `json:"assignees"`
	Labels    []string `json:"labels"`
}
