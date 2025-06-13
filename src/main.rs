use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

struct Project {
    source_dir: String,
    output_dir: String,
}

struct SiteLink {
    name: String,
    url: String,
}

fn replace_template(html: &str, replacements: &[(&str, &str)]) -> String {
    let mut result = html.to_string();
    for (placeholder, content) in replacements {
        result = result.replace(placeholder, content);
    }
    result
}

fn extract_image_paths(html: &str) -> Vec<String> {
    let img_regex = Regex::new(r#"<img[^>]*src=["']([^"'>]+)["'][^>]*>"#).unwrap();
    img_regex
        .captures_iter(html)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect()
}

fn copy_images_flat(
    source_dir: &str,
    output_dir: &str,
    image_paths: &[String],
) -> Result<Vec<String>> {
    let mut copied_images = Vec::new();
    for img_path in image_paths {
        let img_source = Path::new(source_dir).join(img_path);
        if img_source.exists() {
            // Only use the filename, not the path
            let filename = Path::new(img_path).file_name().unwrap();
            let dest_path = Path::new(output_dir).join(filename);
            fs::create_dir_all(output_dir)?;
            fs::copy(&img_source, &dest_path)?;
            copied_images.push(filename.to_string_lossy().to_string());
        }
    }
    Ok(copied_images)
}

fn fix_image_paths(html: &str, image_paths: &[String]) -> String {
    let mut result = html.to_string();
    for img_path in image_paths {
        let filename = Path::new(img_path).file_name().unwrap().to_string_lossy();
        let pattern = format!(r#"src=["'][^"'>]*{}["']"#, regex::escape(&filename));
        let replacement = format!(r#"src="./{}""#, filename);
        result = Regex::new(&pattern)
            .unwrap()
            .replace_all(&result, replacement)
            .to_string();
    }
    result
}

fn wrap_code_blocks(html: &str) -> String {
    // Wrap <pre>...</pre> with <div class="code-block">...</div>
    let re = Regex::new(r"(<pre[\s\S]*?</pre>)").unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        format!("<div class=\"code-block\">{}</div>", &caps[1])
    })
    .to_string()
}

fn add_syntax_highlighting(html: &str) -> String {
    // Add language classes to code blocks for Prism.js
    // This matches <pre><code class="language-xyz"> pattern from markdown
    let re = Regex::new(r#"<pre><code class="language-([^"]+)">"#).unwrap();
    let result = re.replace_all(
        html,
        r#"<pre class="language-$1"><code class="language-$1">"#,
    );

    // Also handle plain code blocks without language specification
    let plain_re = Regex::new(r"<pre><code>").unwrap();
    plain_re
        .replace_all(
            &result,
            r#"<pre class="language-none"><code class="language-none">"#,
        )
        .to_string()
}

fn inject_copy_button(html: &str) -> String {
    // Insert a <button class="copy-btn">Copy</button> as the first child of every <pre> block
    let re = Regex::new(r"(<pre[^>]*>)").unwrap();
    re.replace_all(html, "$1<button class=\"copy-btn\">Copy</button>")
        .to_string()
}

fn process_project(project: &Project, layout: &str) -> Result<()> {
    let readme_path = format!("{}/README.md", project.source_dir);
    let readme_content = fs::read_to_string(&readme_path)?;

    let html_content = markdown::to_html(&readme_content);
    let html_content = add_syntax_highlighting(&html_content);
    let html_content = inject_copy_button(&html_content);
    let html_content = wrap_code_blocks(&html_content);

    let image_paths = extract_image_paths(&html_content);

    fs::create_dir_all(&project.output_dir)?;

    let copied_images = copy_images_flat(&project.source_dir, &project.output_dir, &image_paths)?;

    let html_with_fixed_paths = fix_image_paths(&html_content, &copied_images);

    let final_html = replace_template(layout, &[("{{ main_content }}", &html_with_fixed_paths)]);

    let output_file = format!("{}/index.html", project.output_dir);
    fs::write(output_file, final_html)?;

    Ok(())
}

fn main() -> Result<()> {
    let projects = vec![
        Project {
            source_dir: "../reservoir".to_string(),
            output_dir: "docs/projects/reservoir".to_string(),
        },
        Project {
            source_dir: "../md-chat".to_string(),
            output_dir: "docs/projects/md-chat".to_string(),
        },
        Project {
            source_dir: "../exp-013-service-pipe".to_string(),
            output_dir: "docs/projects/exp-013-service-pipe".to_string(),
        },
    ];

    let links = vec![
        SiteLink {
            name: "exp-013-service-pipe".to_string(),
            url: "/projects/exp-013-service-pipe".to_string(),
        },
        SiteLink {
            name: "Reservoir".to_string(),
            url: "/projects/reservoir".to_string(),
        },
        SiteLink {
            name: "md-chat".to_string(),
            url: "/projects/md-chat".to_string(),
        },
    ];

    let menu_html = links
        .iter()
        .map(|link| format!("<li><a href=\"{}\">{}</a></li>", link.url, link.name))
        .collect::<Vec<_>>()
        .join("\n");

    let layout_template = include_str!("./templates/layout.html");
    let layout = replace_template(layout_template, &[("{{ menu_items }}", &menu_html)]);

    for project in &projects {
        process_project(project, &layout)?;
    }

    let index_content = include_str!("./pages/index.html");
    let index_html = replace_template(&layout, &[("{{ main_content }}", index_content)]);
    fs::write("docs/index.html", index_html)?;

    Ok(())
}
