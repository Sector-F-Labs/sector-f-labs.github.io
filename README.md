# sectorflabs.com

## Rust Static Site Generator

This project includes a custom Rust program (see `src/main.rs`) that generates a static website from project documentation. The generator performs the following tasks:

- Reads project README files (e.g., `reservoir/README.md`).
- Converts Markdown content to HTML and injects it into a common layout template.
- Copies all images referenced in the README (both Markdown and HTML `<img>` tags) to the output folder, ensuring they are available for the generated site.
- Rewrites image paths in the generated HTML so they point to the correct location in the output.
- Generates a navigation menu and index page.
- Outputs the final static site to the `docs/` directory, ready for deployment (e.g., GitHub Pages).

This approach ensures that project documentation and assets are consistently and correctly published as a static website.
