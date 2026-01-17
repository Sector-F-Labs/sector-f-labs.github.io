use crate::git;
use crate::html;
use crate::images;
use crate::templates;
use std::error::Error;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Represents a project to be processed by the static site generator.
///
/// A project contains information about source and output directories,
/// as well as an optional GitHub repository URL for external projects.
#[derive(Debug, Clone)]
pub struct Project {
    /// The source directory containing the project's README.md and assets
    pub source_dir: String,
    /// The output directory where the generated HTML will be placed
    pub output_dir: String,
    /// Optional GitHub repository URL for external projects
    pub github_url: Option<String>,
}

impl Project {
    /// Creates a new Project instance.
    ///
    /// # Arguments
    /// * `source_dir` - The source directory path
    /// * `output_dir` - The output directory path
    ///
    /// # Returns
    /// * `Project` - A new project instance with no GitHub URL
    pub fn new(source_dir: String, output_dir: String) -> Self {
        Self {
            source_dir,
            output_dir,
            github_url: None,
        }
    }

    /// Creates a new Project instance with a GitHub URL.
    ///
    /// # Arguments
    /// * `source_dir` - The source directory path
    /// * `output_dir` - The output directory path
    /// * `github_url` - The GitHub repository URL
    ///
    /// # Returns
    /// * `Project` - A new project instance with the specified GitHub URL
    pub fn with_github_url(source_dir: String, output_dir: String, github_url: String) -> Self {
        Self {
            source_dir,
            output_dir,
            github_url: Some(github_url),
        }
    }

    /// Checks if this project is an external project (source directory starts with "../").
    ///
    /// # Returns
    /// * `bool` - True if the project is external, false otherwise
    pub fn is_external(&self) -> bool {
        self.source_dir.starts_with("../")
    }

    /// Attempts to fetch the Git remote URL for this project if it's external.
    ///
    /// This method will only attempt to fetch the Git remote URL if:
    /// 1. The project is external (source_dir starts with "../")
    /// 2. The source directory is a Git repository
    ///
    /// # Returns
    /// * `Option<String>` - The Git remote URL if found, None otherwise
    pub fn fetch_git_remote(&self) -> Option<String> {
        if self.is_external() && git::is_git_repository(&self.source_dir) {
            git::get_git_remote_url(&self.source_dir)
        } else {
            None
        }
    }

    /// Sets the GitHub URL for this project.
    ///
    /// # Arguments
    /// * `url` - The GitHub repository URL
    pub fn set_github_url(&mut self, url: String) {
        self.github_url = Some(url);
    }

    /// Gets the project name from the output directory path.
    ///
    /// Extracts the last component of the output directory path as the project name.
    ///
    /// # Returns
    /// * `&str` - The project name
    pub fn name(&self) -> &str {
        std::path::Path::new(&self.output_dir)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
    }
}

/// Represents a navigation link in the site menu.
#[derive(Debug, Clone)]
pub struct SiteLink {
    /// Display name for the link
    pub name: String,
    /// URL path for the link
    pub url: String,
}

impl SiteLink {
    /// Creates a new SiteLink.
    ///
    /// # Arguments
    /// * `name` - The display name for the link
    /// * `url` - The URL path for the link
    ///
    /// # Returns
    /// * `SiteLink` - A new site link instance
    pub fn new(name: String, url: String) -> Self {
        Self { name, url }
    }
}

/// Processes a single project: converts README to HTML and handles assets.
///
/// This function performs the complete project processing workflow:
/// 1. Reads the project's README.md file
/// 2. Converts markdown to HTML
/// 3. Processes code blocks with syntax highlighting and copy buttons
/// 4. Extracts and copies images with flattened paths
/// 5. Adds GitHub repository link if available
/// 6. Applies the site layout template
/// 7. Writes the final HTML to the output directory
///
/// # Arguments
/// * `project` - The project to process
/// * `layout` - The HTML layout template
///
/// # Returns
/// * `Result<()>` - Success or error result
///
/// # Errors
/// Returns an error if:
/// * README.md file cannot be read
/// * Output directory cannot be created
/// * Images cannot be copied
/// * Output HTML file cannot be written
pub fn process_project(project: &Project, layout: &str) -> Result<()> {
    // Read and convert README.md to HTML
    let readme_path = format!("{}/README.md", project.source_dir);
    let readme_content = fs::read_to_string(&readme_path)
        .map_err(|e| format!("Failed to read README.md from {}: {}", readme_path, e))?;

    // Convert markdown to HTML and process code blocks
    let html_content = markdown::to_html(&readme_content);
    let html_content = html::process_code_blocks(&html_content);

    // Create output directory
    fs::create_dir_all(&project.output_dir).map_err(|e| {
        format!(
            "Failed to create output directory {}: {}",
            project.output_dir, e
        )
    })?;

    // Process images: extract, copy, and fix paths
    let html_with_images =
        images::process_images(&html_content, &project.source_dir, &project.output_dir).map_err(
            |e| {
                format!(
                    "Failed to process images for project {}: {}",
                    project.name(),
                    e
                )
            },
        )?;

    // Add GitHub link section at the bottom if available
    let github_section = html::create_github_link_section(&project.github_url);
    let main_content = format!("{}{}", html_with_images, github_section);

    // Apply layout template
    let final_html = templates::replace_template(layout, &[("{{ main_content }}", &main_content)]);

    // Write output HTML file
    let output_file = format!("{}/index.html", project.output_dir);
    fs::write(&output_file, final_html)
        .map_err(|e| format!("Failed to write output file {}: {}", output_file, e))?;

    println!(
        "✓ Processed project: {} -> {}",
        project.source_dir, project.output_dir
    );
    Ok(())
}

/// Processes multiple projects with automatic Git remote detection.
///
/// This function processes a collection of projects, automatically detecting
/// and setting GitHub repository URLs for external projects.
///
/// # Arguments
/// * `projects` - A mutable slice of projects to process
/// * `layout` - The HTML layout template
///
/// # Returns
/// * `Result<()>` - Success or error result
pub fn process_projects(projects: &mut [Project], layout: &str) -> Result<()> {
    // Fetch GitHub URLs for external projects
    for project in projects.iter_mut() {
        if project.is_external() {
            if let Some(github_url) = project.fetch_git_remote() {
                project.set_github_url(github_url);
                println!(
                    "📎 Found Git remote for {}: {:?}",
                    project.name(),
                    project.github_url
                );
            }
        }
    }

    // Process each project
    let project_count = projects.len();
    for project in projects {
        if let Err(e) = process_project(project, layout) {
            eprintln!("❌ Error processing project {}: {}", project.name(), e);
            return Err(e);
        }
    }

    println!("✅ Successfully processed {} projects", project_count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_new() {
        let project = Project::new("src".to_string(), "dist".to_string());
        assert_eq!(project.source_dir, "src");
        assert_eq!(project.output_dir, "dist");
        assert!(project.github_url.is_none());
    }

    #[test]
    fn test_project_with_github_url() {
        let project = Project::with_github_url(
            "src".to_string(),
            "dist".to_string(),
            "https://github.com/user/repo".to_string(),
        );
        assert_eq!(
            project.github_url,
            Some("https://github.com/user/repo".to_string())
        );
    }

    #[test]
    fn test_project_is_external() {
        let external = Project::new("../external-project".to_string(), "dist".to_string());
        assert!(external.is_external());

        let internal = Project::new("./internal-project".to_string(), "dist".to_string());
        assert!(!internal.is_external());
    }

    #[test]
    fn test_project_name() {
        let project = Project::new("src".to_string(), "dist/projects/my-project".to_string());
        assert_eq!(project.name(), "my-project");
    }

    #[test]
    fn test_site_link_new() {
        let link = SiteLink::new("Home".to_string(), "/".to_string());
        assert_eq!(link.name, "Home");
        assert_eq!(link.url, "/");
    }

    #[test]
    fn test_project_set_github_url() {
        let mut project = Project::new("src".to_string(), "dist".to_string());
        project.set_github_url("https://github.com/user/repo".to_string());
        assert_eq!(
            project.github_url,
            Some("https://github.com/user/repo".to_string())
        );
    }
}
