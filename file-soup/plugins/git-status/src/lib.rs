use fslint_plugin_api::{Plugin, PluginContext, PluginError, PluginMetadata, PluginResult, PluginStatus};
use git2::{Repository, Status, StatusOptions};
use std::collections::HashMap;
use std::path::Path;

pub struct GitStatusPlugin;

impl GitStatusPlugin {
    pub fn new() -> Self {
        Self
    }

    fn find_repository(path: &Path) -> Option<Repository> {
        // git2 has its own internal caching, so we don't need to cache
        Repository::discover(path).ok()
    }

    fn get_file_status(repo: &Repository, path: &Path) -> Result<Status, git2::Error> {
        let workdir = repo.workdir().ok_or_else(|| {
            git2::Error::from_str("Repository has no working directory")
        })?;

        let relative_path = path.strip_prefix(workdir)
            .map_err(|_| git2::Error::from_str("Path not in repository"))?;

        let mut opts = StatusOptions::new();
        opts.pathspec(relative_path);
        opts.include_untracked(true);
        opts.include_ignored(false);

        let statuses = repo.statuses(Some(&mut opts))?;

        if let Some(entry) = statuses.iter().next() {
            Ok(entry.status())
        } else {
            Ok(Status::CURRENT)
        }
    }

    fn get_branch_name(repo: &Repository) -> Option<String> {
        repo.head().ok()
            .and_then(|head| head.shorthand().map(|s| s.to_string()))
    }

    fn status_to_message(status: Status) -> (String, String, PluginStatus) {
        if status.is_wt_new() || status.is_index_new() {
            ("New".to_string(), "green".to_string(), PluginStatus::Active)
        } else if status.is_wt_modified() || status.is_index_modified() {
            ("Modified".to_string(), "yellow".to_string(), PluginStatus::Alert)
        } else if status.is_wt_deleted() || status.is_index_deleted() {
            ("Deleted".to_string(), "red".to_string(), PluginStatus::Warning)
        } else if status.is_wt_renamed() || status.is_index_renamed() {
            ("Renamed".to_string(), "blue".to_string(), PluginStatus::Active)
        } else if status.is_conflicted() {
            ("Conflict".to_string(), "red".to_string(), PluginStatus::Error)
        } else if status.is_ignored() {
            ("Ignored".to_string(), "gray".to_string(), PluginStatus::Inactive)
        } else {
            ("Clean".to_string(), "gray".to_string(), PluginStatus::Inactive)
        }
    }
}

impl Default for GitStatusPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for GitStatusPlugin {
    fn metadata() -> PluginMetadata {
        PluginMetadata {
            name: "git-status".to_string(),
            version: "0.1.0".to_string(),
            description: "Shows git repository status and branch information".to_string(),
            author: Some("FSLint Contributors".to_string()),
            enabled_by_default: true,
        }
    }

    fn check(&self, context: &PluginContext) -> Result<PluginResult, PluginError> {
        // Find repository
        let repo = match Self::find_repository(&context.path) {
            Some(repo) => repo,
            None => {
                return Ok(PluginResult::inactive("git-status"));
            }
        };

        // Get branch name
        let branch = Self::get_branch_name(&repo)
            .unwrap_or_else(|| "detached".to_string());

        // Get file status
        let status = match Self::get_file_status(&repo, &context.path) {
            Ok(status) => status,
            Err(_) => {
                // File might be outside repo or other error
                return Ok(PluginResult::inactive("git-status"));
            }
        };

        let (message, color, plugin_status) = Self::status_to_message(status);

        let mut result = PluginResult {
            plugin_name: "git-status".to_string(),
            status: plugin_status,
            message: Some(message.clone()),
            color: Some(color),
            tags: vec!["git".to_string()],
            metadata: HashMap::new(),
        };

        result.metadata.insert("branch".to_string(), branch);
        result.metadata.insert("status".to_string(), message);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let metadata = GitStatusPlugin::metadata();
        assert_eq!(metadata.name, "git-status");
        assert_eq!(metadata.version, "0.1.0");
        assert!(metadata.enabled_by_default);
    }
}
