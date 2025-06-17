mod git;
mod html;
mod images;
mod project;
mod templates;

use project::{Project, SiteLink};
use std::error::Error;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    // Define projects to process
    let mut projects = vec![
        Project::new(
            "../reservoir".to_string(),
            "docs/projects/reservoir".to_string(),
        ),
        Project::new(
            "../md-chat".to_string(),
            "docs/projects/md-chat".to_string(),
        ),
        Project::new(
            "../exp-013-service-pipe".to_string(),
            "docs/projects/exp-013-service-pipe".to_string(),
        ),
    ];

    // Define site navigation links
    let links = vec![
        SiteLink::new(
            "exp-013-service-pipe".to_string(),
            "/projects/exp-013-service-pipe".to_string(),
        ),
        SiteLink::new("Reservoir".to_string(), "/projects/reservoir".to_string()),
        SiteLink::new("md-chat".to_string(), "/projects/md-chat".to_string()),
    ];

    // Generate navigation menu HTML
    let menu_links: Vec<(&str, &str)> = links
        .iter()
        .map(|link| (link.name.as_str(), link.url.as_str()))
        .collect();
    let menu_html = templates::create_menu_html(&menu_links);

    // Load and process layout template
    let layout_template = include_str!("./templates/layout.html");
    let layout = templates::replace_template(layout_template, &[("{{ menu_items }}", &menu_html)]);

    // Process all projects
    project::process_projects(&mut projects, &layout)?;

    // Generate index page
    let index_content = include_str!("./pages/index.html");
    let index_html = templates::process_template(&layout, index_content, &menu_html);
    fs::write("docs/index.html", index_html)?;

    println!("🎉 Site generation completed successfully!");
    Ok(())
}
