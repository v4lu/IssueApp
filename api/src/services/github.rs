use std::sync::Arc;

use octocrab::models::Repository;

use crate::errors::CustomError;

pub struct GithubService {
    octocrab: Arc<octocrab::Octocrab>,
}

impl GithubService {
    pub fn new(access_token: &str) -> Result<Self, CustomError> {
        let octocrab = octocrab::OctocrabBuilder::new()
            .personal_token(access_token.to_string())
            .build()
            .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?;

        Ok(Self {
            octocrab: Arc::new(octocrab),
        })
    }

    pub async fn get_org_repos(&self, org_name: &str) -> Result<Vec<Repository>, CustomError> {
        let mut all_repos = Vec::new();

        let mut page = self
            .octocrab
            .orgs(org_name)
            .list_repos()
            .per_page(100) // Maximum items per page
            .send()
            .await
            .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?;

        all_repos.extend(page.items);

        while let Some(next_page) = self
            .octocrab
            .get_page::<Repository>(&page.next)
            .await
            .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?
        {
            let octocrab::Page { items, next, .. } = next_page;
            all_repos.extend(items);
            page.next = next;

            if page.next.is_none() {
                break;
            }
        }

        Ok(all_repos)
    }

    pub async fn get_repository(
        &self,
        owner: &str,
        repo_name: &str,
    ) -> Result<Repository, CustomError> {
        self.octocrab
            .repos(owner, repo_name)
            .get()
            .await
            .map_err(|e| {
                log::error!("Error getting repository: {}", e);
                CustomError::ExternalServiceError(e.to_string())
            })
    }

    pub async fn get_repository_issues(
        &self,
        owner: &str,
        repo_name: &str,
    ) -> Result<Vec<octocrab::models::issues::Issue>, CustomError> {
        let mut all_issues = Vec::new();

        let mut page = self
            .octocrab
            .issues(owner, repo_name)
            .list()
            .state(octocrab::params::State::All)
            .per_page(100)
            .send()
            .await
            .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?;

        all_issues.extend(page.items);

        while let Some(next_page) = self
            .octocrab
            .get_page::<octocrab::models::issues::Issue>(&page.next)
            .await
            .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?
        {
            let octocrab::Page { items, next, .. } = next_page;
            all_issues.extend(items);
            page.next = next;

            if page.next.is_none() {
                break;
            }
        }

        Ok(all_issues)
    }

    pub async fn get_issue(
        &self,
        owner: &str,
        repo_name: &str,
        issue_number: u64,
    ) -> Result<octocrab::models::issues::Issue, CustomError> {
        self.octocrab
            .issues(owner, repo_name)
            .get(issue_number)
            .await
            .map_err(|e| CustomError::ExternalServiceError(e.to_string()))
    }

    pub async fn update_issue(
        &self,
        owner: &str,
        repo_name: &str,
        issue_number: u64,
        title: Option<String>,
        body: Option<String>,
        state: Option<octocrab::models::IssueState>,
        assignees: Option<Vec<String>>,
        labels: Option<Vec<String>>,
    ) -> Result<octocrab::models::issues::Issue, CustomError> {
        let issues_handler = self.octocrab.issues(owner, repo_name);
        let mut issue_update = issues_handler
            .update(issue_number)
            .title(title.as_deref().unwrap_or(""))
            .body(body.as_deref().unwrap_or(""));

        let assignees_refs: Option<Vec<String>> = assignees
            .as_ref()
            .map(|a| a.iter().map(|s| s.clone()).collect());
        let labels_refs: Option<Vec<String>> = labels
            .as_ref()
            .map(|l| l.iter().map(|s| s.clone()).collect());

        let state = state.unwrap_or(octocrab::models::IssueState::Open);
        issue_update = issue_update.state(state);

        issue_update = issue_update
            .assignees(assignees_refs.as_deref().unwrap_or(&[]))
            .labels(labels_refs.as_deref().unwrap_or(&[]));

        issue_update
            .send()
            .await
            .map_err(|e| CustomError::ExternalServiceError(e.to_string()))
    }

    pub async fn create_issue(
        &self,
        owner: &str,
        repo_name: &str,
        title: &str,
        body: Option<String>,
        assignees: Option<Vec<String>>,
        labels: Option<Vec<String>>,
    ) -> Result<octocrab::models::issues::Issue, CustomError> {
        let issues_handler = self.octocrab.issues(owner, repo_name);
        let mut issue_create = issues_handler
            .create(title)
            .body(body.as_deref().unwrap_or(""));

        // Convert Vec<String> to Vec<String> directly
        issue_create = issue_create
            .assignees(assignees.unwrap_or_default())
            .labels(labels.unwrap_or_default());

        issue_create
            .send()
            .await
            .map_err(|e| CustomError::ExternalServiceError(e.to_string()))
    }
}
