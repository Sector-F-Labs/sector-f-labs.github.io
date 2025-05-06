use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

struct SiteLink {
    name: String,
    url: String,
}

fn replace_main_content(html: &str, with: &str) -> String {
    html.replace("{{ main_content }}", with)
}

fn replace_menu_items(html: &str, with: &str) -> String {
    html.replace("{{ menu_items }}", with)
}

fn copy_referenced_images(readme_txt: &str, project_dir: &str, output_path: &str) {
    let image_regex = Regex::new(r"!\[[^\]]*\]\(([^)]+)\)").unwrap();
    for cap in image_regex.captures_iter(readme_txt) {
        let img_path = cap.get(1).unwrap().as_str();
        let img_source = Path::new(project_dir).join(img_path);
        if img_source.exists() {
            if let Some(img_file_name) = img_source.file_name() {
                let img_dest = Path::new(output_path).join(img_file_name);
                let _ = fs::copy(&img_source, &img_dest);
            }
        }
    }
}

fn copy_png_images(project_dir: &str, output_path: &str) {
    if let Ok(entries) = fs::read_dir(project_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "png") {
                if let Some(file_name) = path.file_name() {
                    let dest = Path::new(output_path).join(file_name);
                    let _ = fs::copy(&path, &dest);
                }
            }
        }
    }
}

fn rewrite_image_paths(html: &str, image_names: &[String]) -> String {
    let mut new_html = html.to_string();
    for name in image_names {
        // Replace src="<anything>/<name>" or src="<name>" with src="./<name>"
        let re = Regex::new(&format!(r#"([^"]*{})"#, regex::escape(name))).unwrap();
        new_html = re.replace_all(&new_html, format!("./{}", name)).to_string();
    }
    new_html
}

fn create_project_docs(layout: &str, project: (&str, &str)) {
    let root_path = format!("{}/README.md", project.0);
    let readme_txt = fs::read_to_string(&root_path).expect("Unable to read file");
    let readme_html_raw = markdown::to_html(&readme_txt);
    let output_path = format!("docs/projects/{}", project.1);
    let output_file = format!("{}/index.html", output_path);
    fs::create_dir_all(&output_path).expect("Unable to create directory");

    // Collect image names referenced in the README
    let image_regex = Regex::new(r"!\[[^\]]*\]\(([^)]+)\)").unwrap();
    let mut image_names = Vec::new();
    for cap in image_regex.captures_iter(&readme_txt) {
        let img_path = cap.get(1).unwrap().as_str();
        let img_source = Path::new(project.0).join(img_path);
        if img_source.exists() {
            if let Some(img_file_name) = img_source.file_name() {
                let img_dest = Path::new(&output_path).join(img_file_name);
                let _ = fs::copy(&img_source, &img_dest);
                image_names.push(img_file_name.to_string_lossy().to_string());
            }
        }
    }
    copy_png_images(project.0, &output_path);

    // Rewrite image paths in HTML
    let readme_html = rewrite_image_paths(&readme_html_raw, &image_names);
    let readme_html = replace_main_content(layout, &readme_html);
    fs::write(&output_file, readme_html).expect("Unable to write file");
}

fn main() {
    let project_paths = vec![
        ("../reservoir", "reservoir"),
    ];
    let links = vec![
        SiteLink {
            name: "md-chat".to_string(),
            url: "https://github.com/Sector-F-Labs/md-chat".to_string(),
        },
        SiteLink {
            name: "Reservoir".to_string(),
            url: "projects/reservoir".to_string(),
        },
    ];
    let layout = include_str!("./templates/layout.html");
    let menu_html = links.iter().map(|link| {
        format!("<li><a href=\"{}\">{}</a></li>", link.url, link.name)
    }).collect::<Vec<_>>().join("\n");
    let layout = replace_menu_items(layout, &menu_html);
    for path in project_paths {
        create_project_docs(&layout, path);
    }
    let index = include_str!("./pages/index.html");
    let index = replace_main_content(&layout, index);
    fs::write("docs/index.html", index).expect("Unable to write file");
}
