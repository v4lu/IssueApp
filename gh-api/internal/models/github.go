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

// Issue represents a GitHub issue
// @Description GitHub issue information
type Issue struct {
	// Issue number
	Number int `json:"number"`
	// Issue title
	Title string `json:"title"`
	// Issue body content
	Body string `json:"body"`
	// Issue state (open/closed)
	State string `json:"state"`
	// Creation timestamp
	CreatedAt time.Time `json:"created_at"`
	// Last update timestamp
	UpdatedAt time.Time `json:"updated_at"`
	// List of assignees
	Assignees []string `json:"assignees"`
	// List of labels
	Labels []string `json:"labels"`
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
