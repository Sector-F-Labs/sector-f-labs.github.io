/// Template processing utilities for HTML template replacement.
///
/// This module provides functionality for processing HTML templates with placeholder
/// replacement, commonly used in static site generation.

/// Replaces template placeholders with actual content.
///
/// This function takes an HTML template string and a slice of key-value pairs
/// representing placeholders and their replacement content. It performs simple
/// string replacement for each placeholder.
///
/// # Arguments
/// * `html` - The HTML template content containing placeholders
/// * `replacements` - A slice of tuples where each tuple contains (placeholder, replacement_content)
///
/// # Returns
/// * `String` - The processed HTML with all placeholders replaced
///
/// # Examples
/// ```
/// let template = "Hello {{ name }}, welcome to {{ site }}!";
/// let replacements = &[
///     ("{{ name }}", "Alice"),
///     ("{{ site }}", "My Website"),
/// ];
/// let result = replace_template(template, replacements);
/// assert_eq!(result, "Hello Alice, welcome to My Website!");
/// ```
///
/// # Note
/// This function performs simple string replacement and does not provide any
/// escaping or security features. Ensure that replacement content is properly
/// sanitized if it comes from user input.
pub fn replace_template(html: &str, replacements: &[(&str, &str)]) -> String {
    let mut result = html.to_string();
    for (placeholder, content) in replacements {
        result = result.replace(placeholder, content);
    }
    result
}

/// Creates a navigation menu HTML from a list of site links.
///
/// This function generates HTML list items for navigation menus from a collection
/// of site links, commonly used in website headers or sidebars.
///
/// # Arguments
/// * `links` - A slice of tuples containing (name, url) pairs for each link
///
/// # Returns
/// * `String` - HTML string containing `<li><a>` elements for each link
///
/// # Examples
/// ```
/// let links = &[
///     ("Home", "/"),
///     ("About", "/about"),
///     ("Contact", "/contact"),
/// ];
/// let menu = create_menu_html(links);
/// // Returns: <li><a href="/">Home</a></li>\n<li><a href="/about">About</a></li>\n...
/// ```
pub fn create_menu_html(links: &[(&str, &str)]) -> String {
    links
        .iter()
        .map(|(name, url)| format!("<li><a href=\"{}\">{}</a></li>", url, name))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Processes a complete HTML template with content and menu replacements.
///
/// This is a convenience function that combines template replacement for both
/// main content and navigation menu in a single operation.
///
/// # Arguments
/// * `template` - The HTML template string
/// * `main_content` - The main content to insert
/// * `menu_items` - HTML string containing menu items
///
/// # Returns
/// * `String` - The processed HTML template
///
/// # Examples
/// ```
/// let template = "<nav>{{ menu_items }}</nav><main>{{ main_content }}</main>";
/// let content = "<h1>Welcome</h1>";
/// let menu = "<li><a href='/'>Home</a></li>";
/// let result = process_template(template, content, menu);
/// ```
pub fn process_template(template: &str, main_content: &str, menu_items: &str) -> String {
    replace_template(
        template,
        &[
            ("{{ main_content }}", main_content),
            ("{{ menu_items }}", menu_items),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_template() {
        let template = "Hello {{ name }}, welcome to {{ site }}!";
        let replacements = &[("{{ name }}", "Alice"), ("{{ site }}", "My Website")];
        let result = replace_template(template, replacements);
        assert_eq!(result, "Hello Alice, welcome to My Website!");
    }

    #[test]
    fn test_replace_template_no_replacements() {
        let template = "Hello world!";
        let replacements = &[];
        let result = replace_template(template, replacements);
        assert_eq!(result, "Hello world!");
    }

    #[test]
    fn test_replace_template_missing_placeholder() {
        let template = "Hello {{ name }}, welcome!";
        let replacements = &[("{{ age }}", "25")];
        let result = replace_template(template, replacements);
        assert_eq!(result, "Hello {{ name }}, welcome!");
    }

    #[test]
    fn test_create_menu_html() {
        let links = &[("Home", "/"), ("About", "/about")];
        let result = create_menu_html(links);
        let expected = "<li><a href=\"/\">Home</a></li>\n<li><a href=\"/about\">About</a></li>";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_create_menu_html_empty() {
        let links = &[];
        let result = create_menu_html(links);
        assert_eq!(result, "");
    }

    #[test]
    fn test_process_template() {
        let template = "<nav>{{ menu_items }}</nav><main>{{ main_content }}</main>";
        let content = "<h1>Welcome</h1>";
        let menu = "<li><a href='/'>Home</a></li>";
        let result = process_template(template, content, menu);
        let expected = "<nav><li><a href='/'>Home</a></li></nav><main><h1>Welcome</h1></main>";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_replace_template_multiple_occurrences() {
        let template = "{{ greeting }} {{ name }}, {{ greeting }} again {{ name }}!";
        let replacements = &[("{{ greeting }}", "Hello"), ("{{ name }}", "World")];
        let result = replace_template(template, replacements);
        assert_eq!(result, "Hello World, Hello again World!");
    }
}
