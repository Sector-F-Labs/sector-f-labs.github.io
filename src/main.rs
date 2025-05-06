fn replace_main_content(html: &str, with: &str) -> String {
    html.replace("{{ main_content }}", &with)
}

fn replace_menu_items(html: &str, with: &str) -> String {
    html.replace("{{ menu_items }}", &with)
}

struct SiteLink
{
    name: String,
    url: String,
}

fn main() {
    let links = vec![
        SiteLink {
            name: "md-chat".to_string(),
            url: "https://github.com/Sector-F-Labs/md-chat".to_string(),
        },
        SiteLink {
            name: "Reservoir".to_string(),
            url: "https://github.com/Sector-F-Labs/reservoir".to_string(),
        }
    ];
    let layout = include_str!("./templates/layout.html");
    let layout = replace_menu_items(layout, &links.iter().map(|link| {
        format!("<li><a href=\"{}\">{}</a></li>", link.url, link.name)
    }).collect::<Vec<_>>().join("\n"));
    let index = include_str!("./pages/index.html");
    let index = replace_main_content(&layout, index);

    //write to docs/index.html
    std::fs::write("docs/index.html", index).expect("Unable to write file");
}
