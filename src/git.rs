use std::process::Command;

/// Retrieves the Git remote URL for the 'origin' remote in the specified directory.
///
/// This function executes `git remote get-url origin` in the given directory and
/// processes the output to return a web-friendly URL.
///
/// # Arguments
/// * `dir` - The directory path where the Git repository is located
///
/// # Returns
/// * `Some(String)` - The Git remote URL converted to HTTPS format if successful
/// * `None` - If the Git command fails, directory is not a Git repo, or URL format is unsupported
///
/// # Examples
/// ```
/// let url = get_git_remote_url("../my-project");
/// assert_eq!(url, Some("https://github.com/user/repo".to_string()));
/// ```
pub fn get_git_remote_url(dir: &str) -> Option<String> {
    let output = Command::new("git")
        .args(&["remote", "get-url", "origin"])
        .current_dir(dir)
        .output()
        .ok()?;

    if output.status.success() {
        let url = String::from_utf8(output.stdout).ok()?.trim().to_string();

        // Convert SSH URLs to HTTPS for web links
        if url.starts_with("git@github.com:") {
            let repo = url.strip_prefix("git@github.com:")?.strip_suffix(".git")?;
            Some(format!("https://github.com/{}", repo))
        } else if url.starts_with("https://github.com/") {
            Some(url.strip_suffix(".git").unwrap_or(&url).to_string())
        } else {
            // Return other URLs as-is (GitLab, Bitbucket, etc.)
            Some(url.strip_suffix(".git").unwrap_or(&url).to_string())
        }
    } else {
        None
    }
}

/// Checks if a directory is a Git repository by checking for the .git directory.
///
/// # Arguments
/// * `dir` - The directory path to check
///
/// # Returns
/// * `true` - If the directory contains a .git subdirectory
/// * `false` - If the directory does not contain a .git subdirectory
pub fn is_git_repository(dir: &str) -> bool {
    std::path::Path::new(dir).join(".git").exists()
}
