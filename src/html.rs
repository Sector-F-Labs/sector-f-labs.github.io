use regex::Regex;

/// Cleans up code block markup for consistent styling.
///
/// This function normalizes code block HTML by removing language classes
/// that may have been added by markdown processors.
///
/// # Arguments
/// * `html` - The HTML content to process
///
/// # Returns
/// * `String` - The processed HTML with normalized code blocks
pub fn normalize_code_blocks(html: &str) -> String {
    // Remove language classes from code blocks for simpler styling
    let re = Regex::new(r#"<pre><code[^>]*>"#).unwrap();
    re.replace_all(html, "<pre><code>").to_string()
}

/// Injects copy buttons into code blocks.
///
/// This function adds a "Copy" button as the first child of every <pre> block,
/// allowing users to easily copy code snippets.
///
/// # Arguments
/// * `html` - The HTML content to process
///
/// # Returns
/// * `String` - The processed HTML with copy buttons added to code blocks
pub fn inject_copy_button(html: &str) -> String {
    // Insert a <button class="copy-btn">Copy</button> as the first child of every <pre> block
    let re = Regex::new(r"(<pre[^>]*>)").unwrap();
    re.replace_all(html, "$1<button class=\"copy-btn\">Copy</button>")
        .to_string()
}

/// Wraps code blocks with additional div containers for styling.
///
/// This function wraps each <pre>...</pre> block with a <div class="code-block">...</div>
/// container to provide additional styling hooks.
///
/// # Arguments
/// * `html` - The HTML content to process
///
/// # Returns
/// * `String` - The processed HTML with code blocks wrapped in divs
pub fn wrap_code_blocks(html: &str) -> String {
    // Wrap <pre>...</pre> with <div class="code-block">...</div>
    let re = Regex::new(r"(<pre[\s\S]*?</pre>)").unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        format!("<div class=\"code-block\">{}</div>", &caps[1])
    })
    .to_string()
}

/// Processes HTML content to add code block enhancements.
///
/// This is a convenience function that applies all code block processing in the correct order:
/// 1. Normalizes code block markup
/// 2. Injects copy buttons
/// 3. Wraps code blocks in containers
///
/// # Arguments
/// * `html` - The HTML content to process
///
/// # Returns
/// * `String` - The fully processed HTML with all code block enhancements
pub fn process_code_blocks(html: &str) -> String {
    let html = normalize_code_blocks(html);
    let html = inject_copy_button(&html);
    wrap_code_blocks(&html)
}

/// Creates a GitHub repository link section for project pages.
///
/// This function generates an HTML section with a link to the GitHub repository
/// if a URL is provided.
///
/// # Arguments
/// * `github_url` - Optional GitHub repository URL
///
/// # Returns
/// * `String` - HTML section with GitHub link, or empty string if no URL provided
pub fn create_github_link_section(github_url: &Option<String>) -> String {
    if let Some(url) = github_url {
        format!(
            r#"<div class="github-link">
                <p><strong><i class="fab fa-github"></i> GitHub Repository:</strong> <a href="{}" target="_blank" rel="noopener noreferrer">{}</a></p>
            </div>
            <hr style="margin: 2rem 0;">"#,
            url, url
        )
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_code_blocks() {
        let html = r#"<pre><code class="language-rust">fn main() {}</code></pre>"#;
        let result = normalize_code_blocks(html);
        assert!(result.contains("<pre><code>fn main() {}</code></pre>"));
    }

    #[test]
    fn test_inject_copy_button() {
        let html = r#"<pre><code>some code</code></pre>"#;
        let result = inject_copy_button(html);
        assert!(result.contains(r#"<pre><button class="copy-btn">Copy</button>"#));
    }

    #[test]
    fn test_wrap_code_blocks() {
        let html = r#"<pre><code>some code</code></pre>"#;
        let result = wrap_code_blocks(html);
        assert!(
            result.contains(r#"<div class="code-block"><pre><code>some code</code></pre></div>"#)
        );
    }

    #[test]
    fn test_create_github_link_section() {
        let url = Some("https://github.com/user/repo".to_string());
        let result = create_github_link_section(&url);
        assert!(result.contains("<i class=\"fab fa-github\"></i> GitHub Repository:"));
        assert!(result.contains("https://github.com/user/repo"));

        let no_url: Option<String> = None;
        let empty_result = create_github_link_section(&no_url);
        assert_eq!(empty_result, "");
    }
}
